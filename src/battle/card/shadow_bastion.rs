use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct ShadowBastion {
    increase: f32,
}

impl CardInfo for ShadowBastion {
    fn id() -> &'static str {
        "shadow_bastion"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Hp, CardBranch::Evasion]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Теневой бастион"
    }

    fn desc() -> &'static str {
        "За каждые 400 потерянного здоровья увеличивает шанс уклонения на 1%/2%/3%/4%/6%"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<ShadowBastion> {
    fn effect(&self) -> Box<dyn Effect> {
        ShadowBastion {
            increase: match self.level {
                1 => 0.01,
                2 => 0.02,
                3 => 0.03,
                4 => 0.04,
                5 => 0.06,
                _ => unreachable!(),
            },
        }
        .into()
    }
}

impl Effect for ShadowBastion {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        let count = (myself.hp_lost / 400.0) as u32;
        vec![ModifierDesc {
            modifier: Modifier::AffectEvasion(self.increase * count as f32),
            target: Target::Myself,
            value_kind: ValueKind::Units,
        }]
    }
}
