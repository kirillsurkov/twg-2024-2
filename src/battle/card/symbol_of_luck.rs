use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct SymbolOfLuck {
    increase: f32,
}

impl CardInfo for SymbolOfLuck {
    fn id() -> &'static str {
        "symbol_of_luck"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Crit]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Символ удачи"
    }

    fn desc() -> &'static str {
        "Увеличивает шанс крита на 2%/4%/6%/8%/12%"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<SymbolOfLuck> {
    fn effect(&self) -> Box<dyn Effect> {
        SymbolOfLuck {
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

impl Effect for SymbolOfLuck {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        vec![ModifierDesc {
            modifier: Modifier::AffectCrit(self.increase),
            target: Target::Myself,
            value_kind: ValueKind::Units,
        }]
    }
}
