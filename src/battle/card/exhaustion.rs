use rand::random;

use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct Exhaustion {
    chance: f32,
}

impl CardInfo for Exhaustion {
    fn id() -> &'static str {
        "exhaustion"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Regen]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Истощение"
    }

    fn desc() -> &'static str {
        "Шанс 8%/16%/24%/32%/48% отменить восстановление противника"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<Exhaustion> {
    fn effect(&self) -> Box<dyn Effect> {
        Exhaustion {
            chance: match self.level {
                1 => 8.0,
                2 => 16.0,
                3 => 24.0,
                4 => 32.0,
                5 => 48.0,
                _ => unreachable!(),
            },
        }
        .into()
    }
}

impl Effect for Exhaustion {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        let mut modifiers = vec![];
        for regen in &enemy.procs.regen {
            if random::<f32>() <= self.chance {
                modifiers.push(ModifierDesc {
                    modifier: Modifier::AffectHP(-regen),
                    target: Target::Enemy,
                    value_kind: ValueKind::Units,
                });
            }
        }
        modifiers
    }
}
