use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct ShooterLuck {
    max: f32,
    instances: Vec<f32>,
}

impl CardInfo for ShooterLuck {
    fn id() -> &'static str {
        "shooter_luck"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Crit, CardBranch::Evasion]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Фортуна стрелка"
    }

    fn desc() -> &'static str {
        "Каждое уклонение увеличивает шанс крита на 3% на 3 секунды, максимум до 3%/6%/9%/12%/18%"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<ShooterLuck> {
    fn effect(&self) -> Box<dyn Effect> {
        ShooterLuck {
            max: match self.level {
                1 => 0.03,
                2 => 0.06,
                3 => 0.09,
                4 => 0.12,
                5 => 0.18,
                _ => unreachable!(),
            },
            instances: vec![],
        }
        .into()
    }
}

impl Effect for ShooterLuck {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        self.instances.retain_mut(|timer| {
            *timer += delta;
            *timer >= 3.0
        });
        if myself.procs.evasion {
            self.instances.push(0.0);
        }
        vec![ModifierDesc {
            modifier: Modifier::AffectCrit((self.instances.len() as f32 * 6.0).min(self.max)),
            target: Target::Myself,
            value_kind: ValueKind::Units,
        }]
    }
}
