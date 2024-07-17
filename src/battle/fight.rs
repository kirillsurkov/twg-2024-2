use std::cmp::Ordering;

use rand::random;

use crate::battle::modifier::{Modifier, ModifierDesc, Target};

use super::{card::CardBranch, effect::Effect, hero::Hero, modifier::ValueKind, player::Player};

pub const DURATION: f32 = 60.0;

#[derive(Debug, Clone, Default)]
pub struct Procs {
    pub attack: bool,
    pub ulti: bool,
    pub regen: Vec<f32>,
    pub crit: bool,
    pub evasion: bool,
}

#[derive(Debug, Clone)]
pub struct Branches {
    attack: f32,
    regen: f32,
    hp: f32,
    mana: f32,
    crit: f32,
    evasion: f32,
}

impl Branches {
    fn new(player: &Player) -> Self {
        Self {
            attack: player.branch_value(&CardBranch::Attack) as f32,
            regen: player.branch_value(&CardBranch::Regen) as f32 * 0.5,
            hp: player.branch_value(&CardBranch::Hp) as f32 * 50.0,
            mana: player.branch_value(&CardBranch::Mana) as f32 * 0.1,
            crit: player.branch_value(&CardBranch::Crit) as f32 / 300.0,
            evasion: player.branch_value(&CardBranch::Evasion) as f32 / 300.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Fighter {
    pub hero: Hero,
    pub procs: Procs,
    next_procs: Procs,
    branches: Branches,
    pub hp: f32,
    pub max_hp: f32,
    pub hp_lost: f32,
    pub mana: f32,
    pub mana_regen: f32,
    pub attack: f32,
    pub attack_speed: f32,
    pub crit: f32,
    pub evasion: f32,
    pub ulti_amp: f32,
}

impl Fighter {
    pub fn new(player: &Player) -> Self {
        Self {
            hero: player.hero.clone(),
            procs: Procs::default(),
            next_procs: Procs::default(),
            branches: Branches::new(player),
            hp: player.hero.hp,
            max_hp: player.hero.hp,
            hp_lost: 0.0,
            mana: 0.0,
            mana_regen: player.hero.mana_regen,
            attack: player.hero.attack,
            attack_speed: player.hero.attack_speed,
            crit: player.hero.crit,
            evasion: player.hero.evasion,
            ulti_amp: 1.0,
        }
    }

    fn prepare(&mut self) {
        self.procs = self.next_procs.clone();
        self.next_procs = Procs::default();
        let hp_ratio = self.hp / self.max_hp;
        self.max_hp = self.hero.hp + self.branches.hp;
        self.hp = hp_ratio * self.max_hp;
        self.mana_regen = self.hero.mana_regen + self.branches.mana;
        self.attack = self.hero.attack + self.branches.attack;
        self.attack_speed = self.hero.attack_speed + self.branches.attack * 0.1;
        self.crit = self.hero.crit + self.branches.crit;
        self.evasion = self.hero.evasion + self.branches.evasion;
        self.ulti_amp = 1.0 + self.branches.mana * 0.2;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Owner {
    Fighter1,
    Fighter2,
}

pub struct Fight<'a> {
    effects: Vec<(Box<dyn Effect>, Owner)>,
    player1: &'a mut Player,
    player2: &'a mut Player,
}

#[derive(Debug, Clone)]
pub struct State {
    pub fighter1: Fighter,
    pub fighter2: Fighter,
    pub winner: Option<Owner>,
    pub modifiers: Vec<(Owner, ModifierDesc)>,
}

#[derive(Debug, Default)]
pub struct FightCapture {
    states: Vec<(f32, State)>,
}

impl FightCapture {
    pub fn state(&self, from: f32, to: f32) -> Option<State> {
        let from = self
            .states
            .binary_search_by(|(time, _)| match *time < from {
                true => Ordering::Less,
                false => Ordering::Greater,
            })
            .unwrap_err();

        let to = self
            .states
            .binary_search_by(|(time, _)| match *time > to {
                true => Ordering::Greater,
                false => Ordering::Less,
            })
            .unwrap_err();

        let states = &self.states[from..to];

        states.last().map(|(_, state)| State {
            fighter1: state.fighter1.clone(),
            fighter2: state.fighter2.clone(),
            winner: state.winner,
            modifiers: states
                .iter()
                .flat_map(|(_, state)| state.modifiers.clone())
                .collect(),
        })
    }

    pub fn duration(&self) -> f32 {
        self.states.last().unwrap().0
    }

    pub fn last(&self) -> State {
        self.states.last().unwrap().1.clone()
    }
}

impl<'a> Fight<'a> {
    pub fn new(p1: &'a mut Player, p2: &'a mut Player) -> Self {
        Self {
            effects: vec![]
                .into_iter()
                .chain(p1.hero.abils.iter().map(|a| (a.effect(), Owner::Fighter1)))
                .chain(p2.hero.abils.iter().map(|a| (a.effect(), Owner::Fighter2)))
                .chain(p1.cards.iter().map(|c| (c.effect(), Owner::Fighter1)))
                .chain(p2.cards.iter().map(|c| (c.effect(), Owner::Fighter2)))
                .collect(),
            player1: p1,
            player2: p2,
        }
    }

    pub fn run(&mut self) -> (Owner, FightCapture) {
        let fps = 100.0;
        let delta = 1.0 / fps as f32;

        let mut fighter1 = Fighter::new(self.player1);
        let mut fighter2 = Fighter::new(self.player2);

        let mut capture = FightCapture {
            states: vec![(
                0.0,
                State {
                    fighter1: fighter1.clone(),
                    fighter2: fighter2.clone(),
                    winner: None,
                    modifiers: vec![],
                },
            )],
        };

        let mut winner = None;

        for time in 0..(DURATION * fps) as u32 {
            let time = time as f32 / fps as f32;

            let mut modifiers = self
                .effects
                .iter_mut()
                .flat_map(|(effect, owner)| {
                    let (myself, enemy) = match owner {
                        Owner::Fighter1 => (&mut fighter1, &mut fighter2),
                        Owner::Fighter2 => (&mut fighter2, &mut fighter1),
                    };
                    effect
                        .update(delta, myself, enemy)
                        .into_iter()
                        .map(|m| (*owner, m))
                })
                .collect::<Vec<_>>();
            modifiers.sort_by_key(|(o, m)| (*o, m.key()));

            fighter1.prepare();
            fighter2.prepare();

            for (owner, m) in &modifiers {
                let (mut myself, mut enemy) = match owner {
                    Owner::Fighter1 => (&mut fighter1, &mut fighter2),
                    Owner::Fighter2 => (&mut fighter2, &mut fighter1),
                };
                let ulti_amp = myself.ulti_amp;
                let target = match m.target {
                    Target::Myself => &mut myself,
                    Target::Enemy => &mut enemy,
                };
                match m.modifier {
                    Modifier::AffectAttack(val) => {
                        target.attack = (target.attack + val).max(0.0);
                    }
                    Modifier::AffectAttackSpeed(val) => {
                        target.attack_speed = (target.attack_speed + val).max(0.0);
                    }
                    Modifier::AffectHP(val) => {
                        let val = match m.value_kind {
                            ValueKind::Ulti => ulti_amp * val,
                            _ => val,
                        };
                        let new_hp = (target.hp + val).max(0.0).min(target.max_hp);
                        if val < 0.0 {
                            if random::<f32>() <= target.evasion {
                                target.next_procs.evasion = true;
                            } else {
                                target.hp = new_hp;
                                target.hp_lost -= val;
                            }
                        } else {
                            target.hp = new_hp;
                        }
                    }
                    Modifier::AffectMaxHP(val) => {
                        let ratio = target.hp / target.max_hp;
                        target.max_hp += val;
                        target.hp = target.max_hp * ratio;
                    }
                    Modifier::AffectMana(val) => {
                        target.mana = (target.mana + val).max(0.0).min(100.0);
                    }
                    Modifier::AffectUltiAmp(val) => {
                        target.ulti_amp += val;
                    }
                    Modifier::AffectCrit(val) => {
                        target.crit += val;
                    }
                    Modifier::AffectEvasion(val) => {
                        target.evasion += val;
                    }
                    // markers
                    Modifier::NormalAttack => {
                        myself.next_procs.attack = true;
                    }
                    Modifier::Ulti => {
                        myself.next_procs.ulti = true;
                    }
                    Modifier::Regen(val) => {
                        myself.next_procs.regen.push(val);
                    }
                    Modifier::Crit => {
                        myself.next_procs.crit = true;
                    }
                    Modifier::Evasion => {
                        myself.next_procs.evasion = true;
                    }
                    Modifier::SpawnSwiborg(_) => {}
                    Modifier::ShootSwiborg(_) => {}
                    Modifier::ShootDuck => {}
                    Modifier::SpawnFireCube(_) => {}
                    Modifier::ShootFireCube(_) => {}
                    Modifier::ShootDamageBeam => {}
                    Modifier::ShootHealBeam => {}
                }
            }

            if !modifiers.is_empty() {
                if fighter1.hp <= 0.0 {
                    fighter1.hp = 0.0;
                    winner = Some(Owner::Fighter2);
                } else if fighter2.hp <= 0.0 {
                    fighter2.hp = 0.0;
                    winner = Some(Owner::Fighter1);
                };

                capture.states.push((
                    time,
                    State {
                        fighter1: fighter1.clone(),
                        fighter2: fighter2.clone(),
                        winner: winner.clone(),
                        modifiers,
                    },
                ));

                if winner.is_some() {
                    break;
                }
            }
        }

        let winner = match winner {
            Some(winner) => winner,
            None => {
                let winner = if fighter1.hp < fighter2.hp {
                    Owner::Fighter2
                } else {
                    Owner::Fighter1
                };
                capture.states.push((
                    DURATION,
                    State {
                        fighter1,
                        fighter2,
                        winner: Some(winner),
                        modifiers: vec![],
                    },
                ));
                winner
            }
        };

        let (w, l) = match winner {
            Owner::Fighter1 => (&mut self.player1, &mut self.player2),
            Owner::Fighter2 => (&mut self.player2, &mut self.player1),
        };

        w.money += (w.money / 100).min(10) * 10;
        l.money += (l.money / 100).min(10) * 10;
        w.money += 250;
        l.money += 250;

        w.money += 50;
        l.money += w.attack * 15;

        l.hp = (l.hp - w.attack as i32).max(0);

        l.attack = 3;
        w.attack = (w.attack + 1).min(10);

        (winner, capture)
    }
}
