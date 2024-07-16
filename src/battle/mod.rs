pub mod ability;
pub mod card;
mod effect;
pub mod fight;
pub mod hero;
pub mod modifier;
pub mod player;

use card::decrease_attack::DecreaseAttack;
use card::increase_attack::IncreaseAttack;
use card::increase_attack_speed::IncreaseAttackSpeed;
use card::{Card, CardInfo, CardOps};
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
        cards_pool.add_card::<IncreaseAttack>();
        cards_pool.add_card::<DecreaseAttack>();
        cards_pool.add_card::<IncreaseAttackSpeed>();

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
                Self::reroll_free(&mut self.cards_pool, &mut player);
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
