use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct LifeEssence {
    increase: f32,
}

impl CardInfo for LifeEssence {
    fn id() -> &'static str {
        "life_essence"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Hp]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Эссенция жизни"
    }

    fn desc() -> &'static str {
        "Прибавляет 100/200/300/400/600 здоровья"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<LifeEssence> {
    fn effect(&self) -> Box<dyn Effect> {
        LifeEssence {
            increase: match self.level {
                1 => 100.0,
                2 => 200.0,
                3 => 300.0,
                4 => 400.0,
                5 => 600.0,
                _ => unreachable!(),
            },
        }
        .into()
    }
}

impl Effect for LifeEssence {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        vec![ModifierDesc {
            modifier: Modifier::AffectMaxHP(self.increase),
            target: Target::Myself,
            value_kind: ValueKind::Units,
        }]
    }
}
