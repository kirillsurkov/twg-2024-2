use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct CombatMedkit {
    increase: f32,
}

impl CardInfo for CombatMedkit {
    fn id() -> &'static str {
        "combat_medkit"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Attack, CardBranch::Hp]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Боевой медпак"
    }

    fn desc() -> &'static str {
        "За каждые 400 потерянного здоровья увеличивает базовую атаку на 5/10/15/20/30"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<CombatMedkit> {
    fn effect(&self) -> Box<dyn Effect> {
        CombatMedkit {
            increase: match self.level {
                1 => 5.0,
                2 => 10.0,
                3 => 15.0,
                4 => 20.0,
                5 => 30.0,
                _ => unreachable!(),
            },
        }
        .into()
    }
}

impl Effect for CombatMedkit {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        let count = (myself.hp_lost / 400.0) as u32;
        vec![ModifierDesc {
            modifier: Modifier::AffectAttack(self.increase * count as f32),
            target: Target::Myself,
            value_kind: ValueKind::Units,
        }]
    }
}
