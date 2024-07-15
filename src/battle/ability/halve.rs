use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::Ability;

#[derive(Debug)]
pub struct Halve {
    timer: f32,
    projectiles: Vec<f32>,
}

impl HasEffect for Ability<Halve> {
    fn effect(&self) -> Box<dyn Effect> {
        Halve {
            timer: 0.0,
            projectiles: vec![],
        }
        .into()
    }
}

impl Effect for Halve {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        let mut modifiers = vec![];

        self.projectiles.retain_mut(|timer| {
            *timer += delta;
            if *timer >= 0.5 {
                modifiers.extend(vec![
                    ModifierDesc {
                        modifier: Modifier::AffectHP(-myself.hp * 0.5),
                        target: Target::Myself,
                        value_kind: ValueKind::Units,
                    },
                    ModifierDesc {
                        modifier: Modifier::AffectHP(-myself.hp * 0.5),
                        target: Target::Enemy,
                        value_kind: ValueKind::Units,
                    },
                ]);
                false
            } else {
                true
            }
        });

        if myself.mana >= 100.0 {
            self.projectiles.push(0.0);
            modifiers.extend(vec![
                ModifierDesc {
                    modifier: Modifier::AffectMana(-100.0),
                    target: Target::Myself,
                    value_kind: ValueKind::Units,
                },
                ModifierDesc {
                    modifier: Modifier::ShootDuck,
                    target: Target::Enemy,
                    value_kind: ValueKind::Units,
                },
            ]);
        }

        modifiers
    }
}
