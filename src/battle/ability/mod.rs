use std::marker::PhantomData;

use crate::battle::effect::Effect;

#[derive(Debug)]
pub struct Ability<T: Effect> {
    _pd: PhantomData<T>,
}

impl<T: Effect> Clone for Ability<T> {
    fn clone(&self) -> Self {
        Self { _pd: self._pd }
    }
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

mod regen_mana;
pub use regen_mana::RegenMana;

mod star_wars;
pub use star_wars::StarWars;

mod halve;
pub use halve::Halve;

pub mod fire_cube;
pub use fire_cube::FireCube;

pub mod heal_beam;
pub use heal_beam::HealBeam;