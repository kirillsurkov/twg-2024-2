use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct AgilityWeb {
    decrease: f32,
}

impl CardInfo for AgilityWeb {
    fn id() -> &'static str {
        "agility_web"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Evasion]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Сеть ловкости"
    }

    fn desc() -> &'static str {
        "Уменьшает шанс уклонения врага на 2%/4%/6%/8%/12%"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<AgilityWeb> {
    fn effect(&self) -> Box<dyn Effect> {
        AgilityWeb {
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

impl Effect for AgilityWeb {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        vec![ModifierDesc {
            modifier: Modifier::AffectEvasion(-self.decrease),
            target: Target::Enemy,
            value_kind: ValueKind::Units,
        }]
    }
}
