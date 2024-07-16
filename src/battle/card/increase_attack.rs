use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardInfo};

#[derive(Debug)]
pub struct IncreaseAttack {
    increase: f32,
}

impl CardInfo for IncreaseAttack {
    fn id() -> &'static str {
        "increase_attack"
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Increase attack"
    }

    fn desc() -> &'static str {
        "Increases your attack by 5/10/15/20/30"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<IncreaseAttack> {
    fn effect(&self) -> Box<dyn Effect> {
        IncreaseAttack {
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

impl Effect for IncreaseAttack {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        vec![ModifierDesc {
            modifier: Modifier::AffectAttack(self.increase),
            target: Target::Myself,
            value_kind: ValueKind::Units,
        }]
    }
}
