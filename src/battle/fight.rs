use std::cmp::Ordering;

use crate::battle::modifier::{Modifier, ModifierDesc, Target};

use super::{
    effect::{Effect, Owner},
    player::Player,
};

pub const DURATION: f32 = 10.0;

#[derive(Debug, Clone)]
pub struct Fighter {
    pub hp: f32,
    pub attack: f32,
    pub attack_speed: f32,
}

pub struct Fight {
    fighter1: Fighter,
    fighter2: Fighter,
    effects: Vec<(Box<dyn Effect>, Owner)>,
}

#[derive(Debug, Clone)]
pub struct State {
    fighter1: Fighter,
    fighter2: Fighter,
    modifiers: Vec<(Owner, ModifierDesc)>,
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
}

impl Fight {
    pub fn new(p1: &Player, p2: &Player) -> Self {
        Self {
            fighter1: Fighter {
                hp: p1.hero.hp,
                attack: p1.hero.attack,
                attack_speed: 1.0 / p1.hero.attack_speed,
            },
            fighter2: Fighter {
                hp: p2.hero.hp,
                attack: p2.hero.attack,
                attack_speed: 1.0 / p2.hero.attack_speed,
            },
            effects: vec![]
                .into_iter()
                .chain(p1.hero.abils.iter().map(|a| (a.effect(), Owner::Fighter1)))
                .chain(p2.hero.abils.iter().map(|a| (a.effect(), Owner::Fighter2)))
                .chain(p1.cards.iter().map(|c| (c.effect(), Owner::Fighter1)))
                .chain(p2.cards.iter().map(|c| (c.effect(), Owner::Fighter2)))
                .collect(),
        }
    }

    pub fn run(&mut self) -> FightCapture {
        let fps = 100.0;
        let delta = 1.0 / fps as f32;

        let mut capture = FightCapture::default();

        for time in 0..(DURATION * fps) as u32 {
            let time = time as f32 / fps as f32;

            let mut modifiers = self
                .effects
                .iter_mut()
                .flat_map(|(effect, owner)| {
                    let (myself, enemy) = match owner {
                        Owner::Fighter1 => (&mut self.fighter1, &mut self.fighter2),
                        Owner::Fighter2 => (&mut self.fighter2, &mut self.fighter1),
                    };
                    effect
                        .update(delta, myself, enemy)
                        .into_iter()
                        .map(|m| (*owner, m))
                })
                .collect::<Vec<_>>();
            modifiers.sort_by_key(|(o, m)| (*o, m.key()));

            for (owner, m) in &modifiers {
                let (mut myself, mut enemy) = match owner {
                    Owner::Fighter1 => (&mut self.fighter1, &mut self.fighter2),
                    Owner::Fighter2 => (&mut self.fighter2, &mut self.fighter1),
                };
                let target = match m.target {
                    Target::Myself => &mut myself,
                    Target::Enemy => &mut enemy,
                };
                match m.modifier {
                    Modifier::ChangeAttack(val) => {
                        target.attack += val;
                    }
                    Modifier::Damage(val) => {
                        target.hp -= val;
                    }
                }
            }

            if !modifiers.is_empty() {
                capture.states.push((
                    time,
                    State {
                        fighter1: self.fighter1.clone(),
                        fighter2: self.fighter2.clone(),
                        modifiers,
                    },
                ));
            }
        }

        capture
    }
}
