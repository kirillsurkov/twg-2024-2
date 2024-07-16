use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::Ability;

#[derive(Debug)]
pub struct HealBeam {
    beams: Vec<f32>,
}

impl HasEffect for Ability<HealBeam> {
    fn effect(&self) -> Box<dyn Effect> {
        HealBeam { beams: vec![] }.into()
    }
}

impl Effect for HealBeam {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        let mut modifiers = vec![];

        self.beams.retain_mut(|timer| {
            *timer += delta;
            if *timer >= 1.0 {
                modifiers.push(ModifierDesc {
                    modifier: Modifier::AffectHP(300.0),
                    target: Target::Myself,
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
                    modifier: Modifier::ShootHealBeam,
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
