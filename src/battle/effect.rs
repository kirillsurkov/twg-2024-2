use std::fmt::Debug;

use super::{fight::Fighter, modifier::ModifierDesc};

pub trait Effect: Debug {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc>;
}

impl<T: Effect + 'static> From<T> for Box<dyn Effect> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

pub trait HasEffect: Send + Sync {
    fn effect(&self) -> Box<dyn Effect>;
}

impl<T: HasEffect + 'static> From<T> for Box<dyn HasEffect + Send + Sync> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Owner {
    Fighter1,
    Fighter2,
}
