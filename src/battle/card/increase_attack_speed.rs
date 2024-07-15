use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::Card;

#[derive(Debug)]
pub struct IncreaseAttackSpeed {
    timer: f32,
    increase: f32,
}

impl HasEffect for Card<IncreaseAttackSpeed> {
    fn effect(&self) -> Box<dyn Effect> {
        IncreaseAttackSpeed {
            timer: 0.0,
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

impl Effect for IncreaseAttackSpeed {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        self.timer += delta;
        vec![ModifierDesc {
            modifier: Modifier::AffectAttackSpeed(self.increase * self.timer * 0.1),
            target: Target::Myself,
            value_kind: ValueKind::Units,
        }]
    }
}
