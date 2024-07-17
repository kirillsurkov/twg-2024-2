pub mod ability;
pub mod card;
mod effect;
pub mod fight;
pub mod hero;
pub mod modifier;
pub mod player;

use card::agility_capsule::AgilityCapsule;
use card::agility_web::AgilityWeb;
use card::capture_maneuver::CaptureManeuver;
use card::combat_medkit::CombatMedkit;
use card::energy_drain::EnergyDrain;
use card::energy_source::EnergySource;
use card::exhaustion::Exhaustion;
use card::healing_drone::HealingDrone;
use card::healing_flow::HealingFlow;
use card::hero_might::HeroMight;
use card::illness::Illness;
use card::life_essence::LifeEssence;
use card::life_symbiosis::LifeSymbiosis;
use card::lucky_bullet::LuckyBullet;
use card::magic_generator::MagicGenerator;
use card::mana_crystal::ManaCrystal;
use card::plasma_charge::PlasmaCharge;
use card::plasma_strike::PlasmaStrike;
use card::power_drainer::PowerDrainer;
use card::precision_hit::PrecisionHit;
use card::shadow_bastion::ShadowBastion;
use card::shadow_caster::ShadowCaster;
use card::shadow_dance::ShadowDance;
use card::shock_wave::ShockWave;
use card::shooter_luck::ShooterLuck;
use card::sign_of_misfortune::SignOfMisfortune;
use card::symbol_of_luck::SymbolOfLuck;
use card::{Card, CardBranch, CardInfo, CardOps};
use effect::{Effect, HasEffect};
use fight::{Fight, FightCapture, Owner};
use player::Player;
use rand::prelude::SliceRandom;
use rand::thread_rng;

struct CardsPool {
    players: usize,
    cards: Vec<Box<dyn CardOps>>,
}

impl CardsPool {
    fn new(players: usize) -> Self {
        Self {
            players,
            cards: vec![],
        }
    }

    fn add_card<C>(&mut self)
    where
        C: CardInfo + Effect + 'static,
        Card<C>: HasEffect,
    {
        for _ in 0..self.players {
            for _ in 0..C::max_level() {
                self.cards.push(Box::new(Card::<C>::new(
                    C::id(),
                    C::branches(),
                    C::max_level(),
                    C::name(),
                    C::desc(),
                    C::cost(),
                )));
            }
        }
    }

    fn take(&mut self, player: &Player, n: usize) -> Vec<Box<dyn CardOps>> {
        let mut cards: Vec<Box<dyn CardOps>> = vec![];
        self.cards.shuffle(&mut rand::thread_rng());
        self.cards.retain(|card| {
            let skip = cards.len() >= n
                || cards.iter().find(|c| c.id() == card.id()).map_or_else(
                    || {
                        player
                            .cards
                            .iter()
                            .find(|c| c.id() == card.id())
                            .map_or_else(|| false, |c| c.level() >= c.max_level())
                    },
                    |_| true,
                );
            if !skip {
                cards.push(card.clone());
            }
            skip
        });
        cards
    }

    fn refill(&mut self, cards: Vec<Box<dyn CardOps>>) {
        self.cards.extend(cards);
    }
}

pub struct Battle {
    pub players: Vec<Player>,
    next_players: Vec<Player>,
    cards_pool: CardsPool,
    cards_locked: bool,
}

#[derive(Debug)]
pub struct RoundCapture {
    pub player1: &'static str,
    pub player2: &'static str,
    pub winner: Owner,
    pub fight_capture: FightCapture,
}

impl Battle {
    pub fn new(players: Vec<Player>) -> Self {
        let mut cards_pool = CardsPool::new(players.len());
        cards_pool.add_card::<ShockWave>();
        cards_pool.add_card::<PowerDrainer>();
        cards_pool.add_card::<EnergySource>();
        cards_pool.add_card::<CombatMedkit>();
        cards_pool.add_card::<PlasmaStrike>();
        cards_pool.add_card::<PrecisionHit>();
        cards_pool.add_card::<CaptureManeuver>();
        cards_pool.add_card::<HealingFlow>();
        cards_pool.add_card::<Exhaustion>();
        cards_pool.add_card::<HealingDrone>();
        cards_pool.add_card::<MagicGenerator>();
        cards_pool.add_card::<LuckyBullet>();
        cards_pool.add_card::<AgilityCapsule>();
        cards_pool.add_card::<LifeEssence>();
        cards_pool.add_card::<Illness>();
        cards_pool.add_card::<LifeSymbiosis>();
        cards_pool.add_card::<HeroMight>();
        cards_pool.add_card::<ShadowBastion>();
        cards_pool.add_card::<ManaCrystal>();
        cards_pool.add_card::<EnergyDrain>();
        cards_pool.add_card::<PlasmaCharge>();
        cards_pool.add_card::<ShadowCaster>();
        cards_pool.add_card::<SymbolOfLuck>();
        cards_pool.add_card::<SignOfMisfortune>();
        cards_pool.add_card::<ShooterLuck>();
        cards_pool.add_card::<ShadowDance>();
        cards_pool.add_card::<AgilityWeb>();

        Self {
            players: players
                .into_iter()
                .map(|mut player| {
                    player.reserve_cards(cards_pool.take(&player, 3));
                    player
                })
                .collect(),
            next_players: vec![],
            cards_pool,
            cards_locked: false,
        }
    }

    pub fn round(&mut self) -> Vec<RoundCapture> {
        self.next_players = self.players.clone();

        let mut players = self.next_players.iter_mut().collect::<Vec<_>>();
        players.shuffle(&mut thread_rng());

        let rounds = players
            .chunks_mut(2)
            .into_iter()
            .map(|pair| {
                if let [p1, p2] = pair {
                    let (winner, fight_capture) = Fight::new(p1, p2).run();
                    RoundCapture {
                        player1: p1.hero.id,
                        player2: p2.hero.id,
                        winner,
                        fight_capture,
                    }
                } else {
                    todo!("skipping");
                }
            })
            .collect::<Vec<_>>();

        rounds
    }

    pub fn apply(&mut self) {
        self.players = self
            .next_players
            .iter()
            .map(|player| {
                let mut player = player.clone();
                if !self.cards_locked {
                    Self::reroll_free(&mut self.cards_pool, &mut player);
                }
                player
            })
            .collect();
        self.next_players.clear();
    }

    fn reroll_free(cards_pool: &mut CardsPool, player: &mut Player) {
        let mut cards = vec![];
        cards.append(&mut player.cards_reserved);
        cards_pool.refill(
            cards
                .into_iter()
                .filter_map(|(active, card)| if active { Some(card) } else { None })
                .collect(),
        );
        player.reserve_cards(cards_pool.take(&player, 3));
    }

    pub fn reroll(&mut self, id: &str) {
        let player = self
            .players
            .iter_mut()
            .find(|player| player.hero.id == id)
            .unwrap();
        if player.money >= 20 {
            // player.money -= 20;
            Self::reroll_free(&mut self.cards_pool, player);
        }
    }

    pub fn buy_card(&mut self, id: &str, index: usize) {
        let player = self
            .players
            .iter_mut()
            .find(|player| player.hero.id == id)
            .unwrap();

        player.use_reserved_card(index);

        if !player
            .cards_reserved
            .iter()
            .fold(false, |acc, (a, _)| acc || *a)
        {
            Self::reroll_free(&mut self.cards_pool, player);
        }
    }

    pub fn is_cards_locked(&self) -> bool {
        self.cards_locked
    }

    pub fn set_cards_locked(&mut self, locked: bool) {
        self.cards_locked = locked
    }

    pub fn branch_max(&self, branch: &CardBranch) -> u32 {
        let mut total = 0;
        for player in &self.players {
            total += player.branch_value(branch);
            for (active, card) in &player.cards_reserved {
                if *active && card.branches().contains(branch) {
                    total += 1;
                }
            }
        }
        for card in &self.cards_pool.cards {
            if card.branches().contains(branch) {
                total += 1;
            }
        }
        total / self.players.len() as u32
    }
}

#[test]
fn battle() {
    use hero::{dimas, dtyan, duck, kisanya, nulch, rasp};
    let mut battle = Battle::new(vec![
        Player::new(nulch()),
        Player::new(rasp()),
        Player::new(dtyan()),
        Player::new(dimas()),
        Player::new(duck()),
        Player::new(kisanya()),
    ]);

    println!("{:#?}", battle.round());
}
