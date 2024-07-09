use crate::battle::{
    effect::{Data, Effect, HasEffect},
    modifier::Modifier,
};

use super::Card;

#[derive(Debug)]
pub struct IncreaseAttack {
    increase: f32,
}

impl HasEffect for Card<IncreaseAttack> {
    fn effect(&self) -> Box<dyn Effect> {
        IncreaseAttack {
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

impl Effect for IncreaseAttack {
    fn update(&mut self, _data: Data) -> Vec<Modifier> {
        vec![Modifier::ChangeAttack(self.increase)]
    }
}
