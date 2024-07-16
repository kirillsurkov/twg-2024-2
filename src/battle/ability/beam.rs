use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::Ability;

#[derive(Debug)]
pub struct Beam<const HEAL: bool, const VALUE: u32> {
    beams: Vec<f32>,
}

impl<const HEAL: bool, const VALUE: u32> HasEffect for Ability<Beam<HEAL, VALUE>> {
    fn effect(&self) -> Box<dyn Effect> {
        Beam::<HEAL, VALUE> { beams: vec![] }.into()
    }
}

impl<const HEAL: bool, const VALUE: u32> Effect for Beam<HEAL, VALUE> {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        let mut modifiers = vec![];

        self.beams.retain_mut(|timer| {
            *timer += delta;
            if *timer >= 1.0 {
                modifiers.push(ModifierDesc {
                    modifier: Modifier::AffectHP(VALUE as f32 * if HEAL { 1.0 } else { -1.0 }),
                    target: if HEAL { Target::Myself } else { Target::Enemy },
                    value_kind: ValueKind::Units,
                });
                false
            } else {
                true
            }
        });

        if myself.mana >= 100.0 {
            self.beams.push(0.0);
            modifiers.extend(vec![
                ModifierDesc {
                    modifier: if HEAL {
                        Modifier::ShootHealBeam
                    } else {
                        Modifier::ShootDamageBeam
                    },
                    target: Target::Myself,
                    value_kind: ValueKind::Units,
                },
                ModifierDesc {
                    modifier: Modifier::AffectMana(-100.0),
                    target: Target::Myself,
                    value_kind: ValueKind::Units,
                },
            ]);
        }

        modifiers
    }
}
