use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct HeroMight {
    increase: f32,
}

impl CardInfo for HeroMight {
    fn id() -> &'static str {
        "hero_might"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Hp, CardBranch::Crit]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Сила героя"
    }

    fn desc() -> &'static str {
        "За каждые 400 потерянного здоровья увеличивает шанс крита на 1%/2%/3%/4%/6%"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<HeroMight> {
    fn effect(&self) -> Box<dyn Effect> {
        HeroMight {
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

impl Effect for HeroMight {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        let count = (myself.hp_lost / 400.0) as u32;
        vec![ModifierDesc {
            modifier: Modifier::AffectCrit(self.increase * count as f32),
            target: Target::Myself,
            value_kind: ValueKind::Units,
        }]
    }
}
