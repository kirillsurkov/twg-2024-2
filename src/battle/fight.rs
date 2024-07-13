use std::cmp::Ordering;

use crate::battle::modifier::{Modifier, ModifierDesc, Target};

use super::{effect::Effect, hero::Hero, player::Player};

pub const DURATION: f32 = 10.0;

#[derive(Debug, Clone)]
pub struct Fighter {
    hero: Hero,
    // pub id: String,
    pub hp: f32,
    pub max_hp: f32,
    pub mana: f32,
    pub mana_regen: f32,
    pub attack: f32,
    pub attack_speed: f32,
}

impl Fighter {
    pub fn new(hero: &Hero) -> Self {
        Self {
            hero: hero.clone(),
            hp: hero.hp,
            max_hp: hero.hp,
            mana: 0.0,
            mana_regen: hero.mana_regen,
            attack: hero.attack,
            attack_speed: 1.0 / hero.attack_speed,
        }
    }

    fn prepare(&mut self) {
        self.attack = self.hero.attack;
        self.attack_speed = 1.0 / self.hero.attack_speed;
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

        let mut fighter1 = Fighter::new(&self.player1.hero);
        let mut fighter2 = Fighter::new(&self.player2.hero);

        let mut capture = FightCapture {
            states: vec![(
                0.0,
                State {
                    fighter1: fighter1.clone(),
                    fighter2: fighter2.clone(),
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
                let target = match m.target {
                    Target::Myself => &mut myself,
                    Target::Enemy => &mut enemy,
                };
                match m.modifier {
                    Modifier::ChangeAttack(val) => {
                        target.attack += val;
                    }
                    Modifier::AffectHP(val) => {
                        target.hp += val;
                    }
                    Modifier::AffectMana(val) => {
                        target.mana += val;
                    }
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
                        modifiers,
                    },
                ));

                if winner.is_some() {
                    break;
                }
            }
        }

        let winner = winner.unwrap_or(if fighter1.hp < fighter2.hp {
            Owner::Fighter2
        } else {
            Owner::Fighter1
        });

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

        l.hp = (l.hp - w.attack).max(0);

        l.attack = 3;
        w.attack = (w.attack + 1).min(10);

        (winner, capture)
    }
}
