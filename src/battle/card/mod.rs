use std::marker::PhantomData;

use crate::battle::effect::Effect;

pub struct Card<T: Effect> {
    pub level: u8,
    pub max_level: u8,
    _pd: PhantomData<T>,
}

impl<T: Effect> Clone for Card<T> {
    fn clone(&self) -> Self {
        Self {
            level: self.level,
            max_level: self.max_level,
            _pd: self._pd,
        }
    }
}

impl<T: Effect> Card<T> {
    pub fn new(max_level: u8) -> Self {
        Self {
            level: 1,
            max_level,
            _pd: PhantomData::default(),
        }
    }
}

pub mod decrease_attack;
pub mod increase_attack;
