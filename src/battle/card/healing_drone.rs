use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct HealingDrone {
    rate: f32,
    timer: f32,
}

impl CardInfo for HealingDrone {
    fn id() -> &'static str {
        "healing_drone"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Regen, CardBranch::Hp]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Лечебный дрон"
    }

    fn desc() -> &'static str {
        "Каждую секунду восстанавливает 1%/2%/3%/4%/6% от потерянного здоровья"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<HealingDrone> {
    fn effect(&self) -> Box<dyn Effect> {
        HealingDrone {
            rate: match self.level {
                1 => 0.01,
                2 => 0.02,
                3 => 0.03,
                4 => 0.04,
                5 => 0.06,
                _ => unreachable!(),
            },
            timer: 0.0,
        }
        .into()
    }
}

impl Effect for HealingDrone {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        self.timer += delta;
        if self.timer >= 1.0 {
            let regen = myself.hp_lost * self.rate;
            vec![
                ModifierDesc {
                    modifier: Modifier::AffectHP(regen),
                    target: Target::Myself,
                    value_kind: ValueKind::Units,
                },
                ModifierDesc {
                    modifier: Modifier::Regen(regen),
                    target: Target::Myself,
                    value_kind: ValueKind::Units,
                },
            ]
        } else {
            vec![]
        }
    }
}
