use std::fmt::Debug;

use super::{modifier::Modifier, Fighter};

pub struct Data<'a> {
    pub delta: f32,
    pub myself: &'a Fighter,
    pub enemy: &'a Fighter,
}

pub trait Effect: Debug {
    fn update(&mut self, data: Data) -> Vec<Modifier>;
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
