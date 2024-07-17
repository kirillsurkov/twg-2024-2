use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::{Card, CardBranch, CardInfo};

#[derive(Debug)]
pub struct LifeSymbiosis {
    increase: f32,
}

impl CardInfo for LifeSymbiosis {
    fn id() -> &'static str {
        "life_symbiosis"
    }

    fn branches() -> Vec<CardBranch> {
        vec![CardBranch::Hp, CardBranch::Mana]
    }

    fn max_level() -> u8 {
        5
    }

    fn name() -> &'static str {
        "Симбиоз жизни"
    }

    fn desc() -> &'static str {
        "За каждые 400 потерянного здоровья увеличивает урон от ультимейта на 1%/2%/3%/4%/6%"
    }

    fn cost() -> u32 {
        100
    }
}

impl HasEffect for Card<LifeSymbiosis> {
    fn effect(&self) -> Box<dyn Effect> {
        LifeSymbiosis {
            increase: match self.level {
                1 => 0.01,
                2 => 0.02,
                3 => 0.03,
                4 => 0.04,
                5 => 0.06,
                _ => unreachable!(),
            },
        }
        .into()
    }
}

impl Effect for LifeSymbiosis {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        let count = (myself.hp_lost / 400.0) as u32;
        vec![ModifierDesc {
            modifier: Modifier::AffectUltiAmp(self.increase * count as f32),
            target: Target::Myself,
            value_kind: ValueKind::Units,
        }]
    }
}
