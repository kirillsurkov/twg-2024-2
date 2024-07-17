use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct PowerDrainer {
    decrease: f32,
}

impl CardInfo for PowerDrainer {
    fn id() -> &'static str {
        "power_drainer"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Attack]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Поглотитель силы"
    }

    fn desc() -> &'static str {
        "Уменьшает базовую атаку врага на 5/10/15/20/30"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<PowerDrainer> {
    fn effect(&self) -> Box<dyn Effect> {
        PowerDrainer {
            decrease: match self.level {
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

impl Effect for PowerDrainer {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        vec![ModifierDesc {
            modifier: Modifier::AffectAttack(-self.decrease),
            target: Target::Enemy,
            value_kind: ValueKind::Units,
        }]
    }
}
