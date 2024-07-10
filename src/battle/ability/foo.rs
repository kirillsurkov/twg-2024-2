use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::ModifierDesc,
};

use super::Ability;

#[derive(Debug)]
pub struct Foo {
    timer: f32,
}

impl HasEffect for Ability<Foo> {
    fn effect(&self) -> Box<dyn Effect> {
        Foo { timer: 0.0 }.into()
    }
}

impl Effect for Foo {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        vec![]
    }
}
