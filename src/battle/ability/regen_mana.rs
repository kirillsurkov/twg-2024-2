use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::Ability;

#[derive(Debug)]
pub struct RegenMana {}

impl HasEffect for Ability<RegenMana> {
    fn effect(&self) -> Box<dyn Effect> {
        RegenMana {}.into()
    }
}

impl Effect for RegenMana {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        vec![ModifierDesc {
            modifier: Modifier::AffectMana(myself.mana_regen * delta),
            target: Target::Enemy,
            value_kind: ValueKind::Units,
        }]
    }
}
