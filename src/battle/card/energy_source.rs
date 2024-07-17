use rand::random;

use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct EnergySource {
    regen: f32,
}

impl CardInfo for EnergySource {
    fn id() -> &'static str {
        "energy_source"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Attack, CardBranch::Regen]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Источник энергии"
    }

    fn desc() -> &'static str {
        "С шансом 60% восстанавливает 10/20/30/40/60 здоровья после каждой атаки"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<EnergySource> {
    fn effect(&self) -> Box<dyn Effect> {
        EnergySource {
            regen: match self.level {
                1 => 10.0,
                2 => 20.0,
                3 => 30.0,
                4 => 40.0,
                5 => 60.0,
                _ => unreachable!(),
            },
        }
        .into()
    }
}

impl Effect for EnergySource {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        let mut modifiers = vec![];
        if myself.procs.attack {
            if random::<f32>() <= 0.6 {
                modifiers.extend(vec![ModifierDesc {
                    modifier: Modifier::AffectHP(self.regen),
                    target: Target::Myself,
                    value_kind: ValueKind::Units,
                }]);
            }
        }
        modifiers
    }
}
