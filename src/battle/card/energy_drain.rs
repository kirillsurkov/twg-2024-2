use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct EnergyDrain {
    decrease: f32,
}

impl CardInfo for EnergyDrain {
    fn id() -> &'static str {
        "energy_drain"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Mana]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Поглотитель энергии"
    }

    fn desc() -> &'static str {
        "Ослабляет урон от ультимейта противника на 8%/16%/24%/36%/48%"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<EnergyDrain> {
    fn effect(&self) -> Box<dyn Effect> {
        EnergyDrain {
            decrease: match self.level {
                1 => 0.08,
                2 => 0.16,
                3 => 0.24,
                4 => 0.36,
                5 => 0.48,
                _ => unreachable!(),
            },
        }
        .into()
    }
}

impl Effect for EnergyDrain {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        vec![ModifierDesc {
            modifier: Modifier::AffectUltiAmp(-self.decrease),
            target: Target::Enemy,
            value_kind: ValueKind::Units,
        }]
    }
}
