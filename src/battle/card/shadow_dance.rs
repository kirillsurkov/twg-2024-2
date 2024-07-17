use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct ShadowDance {
    increase: f32,
}

impl CardInfo for ShadowDance {
    fn id() -> &'static str {
        "shadow_dance"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Evasion]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Танец теней"
    }

    fn desc() -> &'static str {
        "Увеличивает шанс уклонения на 2%/4%/6%/8%/12%"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<ShadowDance> {
    fn effect(&self) -> Box<dyn Effect> {
        ShadowDance {
            increase: match self.level {
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

impl Effect for ShadowDance {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        vec![ModifierDesc {
            modifier: Modifier::AffectEvasion(self.increase),
            target: Target::Myself,
            value_kind: ValueKind::Units,
        }]
    }
}
