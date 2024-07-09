use std::marker::PhantomData;

use crate::battle::effect::Effect;

pub struct Ability<T: Effect> {
    _pd: PhantomData<T>,
}

impl<T: Effect> Ability<T> {
    pub fn new() -> Self {
        Self {
            _pd: PhantomData::default(),
        }
    }
}

mod attack;
pub use attack::Attack;

mod foo;