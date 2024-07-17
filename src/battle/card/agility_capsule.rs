use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct AgilityCapsule {
    regen: f32,
}

impl CardInfo for AgilityCapsule {
    fn id() -> &'static str {
        "agility_capsule"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Regen, CardBranch::Evasion]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Капсула ловкости"
    }

    fn desc() -> &'static str {
        "Каждое уклонение восстанавливает 10/20/30/40/60 здоровья"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<AgilityCapsule> {
    fn effect(&self) -> Box<dyn Effect> {
        AgilityCapsule {
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

impl Effect for AgilityCapsule {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        if myself.procs.crit {
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
