use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct ManaCrystal {
    increase: f32,
}

impl CardInfo for ManaCrystal {
    fn id() -> &'static str {
        "mana_crystal"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Mana]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Кристалл маны"
    }

    fn desc() -> &'static str {
        "Увеличивает ультимейт на 10%/20%/30%/40%/60%"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<ManaCrystal> {
    fn effect(&self) -> Box<dyn Effect> {
        ManaCrystal {
            increase: match self.level {
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

impl Effect for ManaCrystal {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        vec![ModifierDesc {
            modifier: Modifier::AffectUltiAmp(self.increase),
            target: Target::Myself,
            value_kind: ValueKind::Units,
        }]
    }
}
