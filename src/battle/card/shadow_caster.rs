use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct ShadowCaster {
    regen: f32,
}

impl CardInfo for ShadowCaster {
    fn id() -> &'static str {
        "shadow_caster"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Mana, CardBranch::Evasion]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Теневая магия"
    }

    fn desc() -> &'static str {
        "Каждое уклонение восстанваливает 1.5/3/4.5/6/9 маны"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<ShadowCaster> {
    fn effect(&self) -> Box<dyn Effect> {
        ShadowCaster {
            regen: match self.level {
                1 => 1.5,
                2 => 3.0,
                3 => 4.5,
                4 => 6.0,
                5 => 9.0,
                _ => unreachable!(),
            },
        }
        .into()
    }
}

impl Effect for ShadowCaster {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        if myself.procs.evasion {
            vec![ModifierDesc {
                modifier: Modifier::AffectMana(self.regen),
                target: Target::Myself,
                value_kind: ValueKind::Units,
            }]
        } else {
            vec![]
        }
    }
}
