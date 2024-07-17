use rand::random;

use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct PlasmaStrike {
    regen: f32,
}

impl CardInfo for PlasmaStrike {
    fn id() -> &'static str {
        "plasma_strike"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Attack, CardBranch::Mana]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Плазменный удар"
    }

    fn desc() -> &'static str {
        "С шансом 60% восстанавливает 1.5/3/4.5/6/9 маны на каждой атаке"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<PlasmaStrike> {
    fn effect(&self) -> Box<dyn Effect> {
        PlasmaStrike {
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

impl Effect for PlasmaStrike {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        let mut modifiers = vec![];
        if myself.procs.attack {
            if random::<f32>() <= 0.6 {
                modifiers.extend(vec![ModifierDesc {
                    modifier: Modifier::AffectMana(self.regen),
                    target: Target::Myself,
                    value_kind: ValueKind::Units,
                }]);
            }
        }
        modifiers
    }
}
