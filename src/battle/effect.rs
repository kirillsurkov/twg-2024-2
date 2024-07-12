use std::fmt::Debug;

use dyn_clone::DynClone;

use super::{fight::Fighter, modifier::ModifierDesc};

pub trait Effect: Debug {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc>;
}

impl<T: Effect + 'static> From<T> for Box<dyn Effect> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

pub trait HasEffect: Debug + Send + Sync + DynClone {
    fn effect(&self) -> Box<dyn Effect>;
}

dyn_clone::clone_trait_object!(HasEffect);

impl<T: HasEffect + 'static> From<T> for Box<dyn HasEffect> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}
