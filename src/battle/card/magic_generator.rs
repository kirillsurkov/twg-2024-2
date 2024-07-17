use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct MagicGenerator {
    regen: f32,
}

impl CardInfo for MagicGenerator {
    fn id() -> &'static str {
        "magic_generator"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Regen, CardBranch::Mana]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Магический генератор"
    }

    fn desc() -> &'static str {
        "Каждый ультимейт восстанавливает 50/100/150/200/300 здоровья"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<MagicGenerator> {
    fn effect(&self) -> Box<dyn Effect> {
        MagicGenerator {
            regen: match self.level {
                1 => 50.0,
                2 => 100.0,
                3 => 150.0,
                4 => 200.0,
                5 => 300.0,
                _ => unreachable!(),
            },
        }
        .into()
    }
}

impl Effect for MagicGenerator {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        if myself.procs.ulti {
            vec![
                ModifierDesc {
                    modifier: Modifier::Regen(self.regen),
                    target: Target::Myself,
                    value_kind: ValueKind::Units,
                },
                ModifierDesc {
                    modifier: Modifier::Regen(self.regen),
                    target: Target::Myself,
                    value_kind: ValueKind::Units,
                },
            ]
        } else {
            vec![]
        }
    }
}
