use crate::battle::{
    effect::{Data, Effect, HasEffect},
    modifier::Modifier,
};

use super::Card;

#[derive(Debug)]
pub struct DecreaseAttack {
    decrease: f32,
}

impl HasEffect for Card<DecreaseAttack> {
    fn effect(&self) -> Box<dyn Effect> {
        DecreaseAttack {
            decrease: match self.level {
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

impl Effect for DecreaseAttack {
    fn update(&mut self, _data: Data) -> Vec<Modifier> {
        vec![Modifier::ChangeAttack(-self.decrease)]
    }
}
