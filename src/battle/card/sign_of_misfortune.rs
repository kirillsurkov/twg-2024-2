use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct SignOfMisfortune {
    decrease: f32,
}

impl CardInfo for SignOfMisfortune {
    fn id() -> &'static str {
        "sign_of_misfortune"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Crit]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Знак проклятия"
    }

    fn desc() -> &'static str {
        "Уменьшает шанс крита врага на 2%/4%/6%/8%/12%"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<SignOfMisfortune> {
    fn effect(&self) -> Box<dyn Effect> {
        SignOfMisfortune {
            decrease: match self.level {
                1 => 0.02,
                2 => 0.04,
                3 => 0.06,
                4 => 0.08,
                5 => 0.12,
                _ => unreachable!(),
            },
        }
        .into()
    }
}

impl Effect for SignOfMisfortune {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        vec![ModifierDesc {
            modifier: Modifier::AffectCrit(-self.decrease),
            target: Target::Enemy,
            value_kind: ValueKind::Units,
        }]
    }
}
