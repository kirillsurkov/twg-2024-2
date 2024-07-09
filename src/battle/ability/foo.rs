use crate::battle::{
    effect::{Data, Effect, HasEffect},
    modifier::Modifier,
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
    fn update(&mut self, data: Data) -> Vec<Modifier> {
        vec![]
    }
}
