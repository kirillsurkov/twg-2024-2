use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct LuckyBullet {
    regen: f32,
}

impl CardInfo for LuckyBullet {
    fn id() -> &'static str {
        "lucky_bullet"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Regen, CardBranch::Crit]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Удачный патрон"
    }

    fn desc() -> &'static str {
        "Каждый крит восстанавливает 10/20/30/40/60 здоровья"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<LuckyBullet> {
    fn effect(&self) -> Box<dyn Effect> {
        LuckyBullet {
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

impl Effect for LuckyBullet {
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
