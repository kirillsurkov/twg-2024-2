use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct HealingFlow {
    regen: f32,
    timer: f32,
}

impl CardInfo for HealingFlow {
    fn id() -> &'static str {
        "healing_flow"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Regen]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Целебный поток"
    }

    fn desc() -> &'static str {
        "Каждую секунду восстанавливает 8/16/24/32/48 здоровья"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<HealingFlow> {
    fn effect(&self) -> Box<dyn Effect> {
        HealingFlow {
            regen: match self.level {
                1 => 8.0,
                2 => 16.0,
                3 => 24.0,
                4 => 32.0,
                5 => 48.0,
                _ => unreachable!(),
            },
            timer: 0.0,
        }
        .into()
    }
}

impl Effect for HealingFlow {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        self.timer += delta;
        if self.timer >= 1.0 {
            self.timer = 0.0;
            vec![
                ModifierDesc {
                    modifier: Modifier::AffectHP(self.regen),
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
