use std::marker::PhantomData;

use crate::battle::effect::Effect;

pub struct Card<T: Effect> {
    pub level: u8,
    pub max_level: u8,
    _pd: PhantomData<T>,
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

pub mod increase_attack;
pub mod decrease_attack;
