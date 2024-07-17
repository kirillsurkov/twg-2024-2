use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct CaptureManeuver {
    rate: f32,
}

impl CardInfo for CaptureManeuver {
    fn id() -> &'static str {
        "capture_maneuver"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Attack, CardBranch::Evasion]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Манёвр захвата"
    }

    fn desc() -> &'static str {
        "Переводит 10%/20%/30%/40%/60% базовой атаки в уклонение"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<CaptureManeuver> {
    fn effect(&self) -> Box<dyn Effect> {
        CaptureManeuver {
            rate: match self.level {
                1 => 0.1,
                2 => 0.2,
                3 => 0.3,
                4 => 0.4,
                5 => 0.6,
                _ => unreachable!(),
            },
        }
        .into()
    }
}

impl Effect for CaptureManeuver {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        vec![ModifierDesc {
            modifier: Modifier::AffectEvasion((myself.attack * self.rate) / 100.0),
            target: Target::Myself,
            value_kind: ValueKind::Units,
        }]
    }
}
