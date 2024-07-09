use crate::battle::{
    effect::{Data, Effect, HasEffect},
    modifier::Modifier,
};

use super::Ability;

#[derive(Debug)]
pub struct Attack {
    timer: f32,
}

impl HasEffect for Ability<Attack> {
    fn effect(&self) -> Box<dyn Effect> {
        Attack { timer: 0.0 }.into()
    }
}

impl Effect for Attack {
    fn update(&mut self, data: Data) -> Vec<Modifier> {
        self.timer += data.delta;
        if self.timer >= data.myself.attack_speed {
            self.timer = 0.0;
            vec![Modifier::Damage(data.myself.attack)]
        } else {
            vec![]
        }
    }
}
