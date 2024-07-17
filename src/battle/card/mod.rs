use std::marker::PhantomData;

use dyn_clone::DynClone;

use crate::battle::effect::Effect;

use super::effect::HasEffect;

#[derive(Debug, Clone, PartialEq)]
pub enum CardBranch {
    Attack,
    Regen,
    Hp,
    Mana,
    Crit,
    Evasion,
}

#[derive(Debug)]
pub struct Card<T: Effect> {
    pub id: &'static str,
    pub branches: Vec<CardBranch>,
    pub level: u8,
    pub max_level: u8,
    pub name: &'static str,
    pub desc: &'static str,
    pub cost: u32,
    _pd: PhantomData<T>,
}

pub trait CardInfo {
    fn id() -> &'static str;
    fn branches() -> Vec<CardBranch>;
    fn max_level() -> u8;
    fn name() -> &'static str;
    fn desc() -> &'static str;
    fn cost() -> u32;
}

pub trait CardOps: HasEffect + DynClone {
    fn id(&self) -> &'static str;
    fn branches(&self) -> Vec<CardBranch>;
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

    fn branches(&self) -> Vec<CardBranch> {
        self.branches.clone()
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
            branches: self.branches.clone(),
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
        branches: Vec<CardBranch>,
        max_level: u8,
        name: &'static str,
        desc: &'static str,
        cost: u32,
    ) -> Self {
        Self {
            id,
            branches,
            level: 1,
            max_level,
            name,
            desc,
            cost,
            _pd: PhantomData::default(),
        }
    }
}

pub mod agility_capsule;
pub mod agility_web;
pub mod capture_maneuver;
pub mod combat_medkit;
pub mod energy_drain;
pub mod energy_source;
pub mod exhaustion;
pub mod healing_drone;
pub mod healing_flow;
pub mod hero_might;
pub mod illness;
pub mod life_essence;
pub mod life_symbiosis;
pub mod lucky_bullet;
pub mod magic_generator;
pub mod mana_crystal;
pub mod plasma_charge;
pub mod plasma_strike;
pub mod power_drainer;
pub mod precision_hit;
pub mod shadow_bastion;
pub mod shadow_caster;
pub mod shadow_dance;
pub mod shock_wave;
pub mod shooter_luck;
pub mod sign_of_misfortune;
pub mod symbol_of_luck;
