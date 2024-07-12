use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
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
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        self.timer += delta;
        if self.timer >= myself.attack_speed {
            self.timer = 0.0;
            vec![ModifierDesc {
                modifier: Modifier::AffectHP(-myself.attack * 6.0),
                target: Target::Enemy,
                value_kind: ValueKind::Units,
            }]
        } else {
            vec![]
        }
    }
}
