use std::marker::PhantomData;

use dyn_clone::DynClone;

use crate::battle::effect::Effect;

use super::effect::HasEffect;

#[derive(Debug)]
pub struct Card<T: Effect> {
    pub id: &'static str,
    pub level: u8,
    pub max_level: u8,
    pub name: &'static str,
    pub desc: &'static str,
    pub cost: u32,
    _pd: PhantomData<T>,
}

pub trait CardInfo {
    fn id() -> &'static str;
    fn max_level() -> u8;
    fn name() -> &'static str;
    fn desc() -> &'static str;
    fn cost() -> u32;
}

pub trait CardOps: HasEffect + DynClone {
    fn id(&self) -> &'static str;
    fn level(&self) -> u8;
    fn max_level(&self) -> u8;
    fn set_level(&mut self, level: u8);
    fn name(&self) -> &'static str;
    fn desc(&self) -> &'static str;
    fn cost(&self) -> u32;
}

dyn_clone::clone_trait_object!(CardOps);

impl<T: Effect> CardOps for Card<T>
where
    Card<T>: HasEffect,
{
    fn id(&self) -> &'static str {
        self.id
    }

    fn level(&self) -> u8 {
        self.level
    }

    fn max_level(&self) -> u8 {
        self.max_level
    }

    fn set_level(&mut self, level: u8) {
        self.level = level;
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn desc(&self) -> &'static str {
        self.desc
    }

    fn cost(&self) -> u32 {
        self.cost
    }
}

impl PartialEq for Box<dyn CardOps> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
            && self.level() == other.level()
            && self.max_level() == other.max_level()
    }
}

impl<T: Effect> Clone for Card<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            level: self.level,
            max_level: self.max_level,
            name: self.name,
            desc: self.desc,
            cost: self.cost,
            _pd: self._pd,
        }
    }
}

impl<T: Effect> Card<T> {
    pub fn new(
        id: &'static str,
        max_level: u8,
        name: &'static str,
        desc: &'static str,
        cost: u32,
    ) -> Self {
        Self {
            level: 1,
            max_level,
            id,
            name,
            desc,
            cost,
            _pd: PhantomData::default(),
        }
    }
}

pub mod decrease_attack;
pub mod increase_attack;
pub mod increase_attack_speed;
