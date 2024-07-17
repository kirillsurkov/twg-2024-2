use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct PrecisionHit {
    max: f32,
    instances: Vec<f32>,
}

impl CardInfo for PrecisionHit {
    fn id() -> &'static str {
        "precision_hit"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Attack, CardBranch::Crit]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Точечный удар"
    }

    fn desc() -> &'static str {
        "Каждый крит увеличивает атаку на 6 на 3 секунды, максимум до 6/12/18/24/40"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<PrecisionHit> {
    fn effect(&self) -> Box<dyn Effect> {
        PrecisionHit {
            max: match self.level {
                1 => 6.0,
                2 => 12.0,
                3 => 18.5,
                4 => 24.0,
                5 => 40.0,
                _ => unreachable!(),
            },
            instances: vec![],
        }
        .into()
    }
}

impl Effect for PrecisionHit {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        self.instances.retain_mut(|timer| {
            *timer += delta;
            *timer >= 3.0
        });
        if myself.procs.crit {
            self.instances.push(0.0);
        }
        vec![ModifierDesc {
            modifier: Modifier::AffectAttack((self.instances.len() as f32 * 6.0).min(self.max)),
            target: Target::Myself,
            value_kind: ValueKind::Units,
        }]
    }
}
