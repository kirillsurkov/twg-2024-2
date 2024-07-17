use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct Illness {
    decrease: f32,
}

impl CardInfo for Illness {
    fn id() -> &'static str {
        "illness"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Hp]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Болезнь"
    }

    fn desc() -> &'static str {
        "Уменьшает максимальное здоровье противника на 100/200/300/400/600"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<Illness> {
    fn effect(&self) -> Box<dyn Effect> {
        Illness {
            decrease: match self.level {
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

impl Effect for Illness {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        vec![ModifierDesc {
            modifier: Modifier::AffectMaxHP(-self.decrease),
            target: Target::Enemy,
            value_kind: ValueKind::Units,
        }]
    }
}
