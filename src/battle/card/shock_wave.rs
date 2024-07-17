use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct ShockWave {
    increase: f32,
}

impl CardInfo for ShockWave {
    fn id() -> &'static str {
        "shock_wave"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Attack]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Ударная волна"
    }

    fn desc() -> &'static str {
        "Увеличивает базовую атаку на 5/10/15/20/30"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<ShockWave> {
    fn effect(&self) -> Box<dyn Effect> {
        ShockWave {
            increase: match self.level {
                1 => 5.0,
                2 => 10.0,
                3 => 15.0,
                4 => 20.0,
                5 => 30.0,
                _ => unreachable!(),
            },
        }
        .into()
    }
}

impl Effect for ShockWave {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        vec![ModifierDesc {
            modifier: Modifier::AffectAttack(self.increase),
            target: Target::Myself,
            value_kind: ValueKind::Units,
        }]
    }
}
