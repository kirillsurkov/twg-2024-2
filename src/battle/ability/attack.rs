use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::Ability;

#[derive(Debug)]
pub struct Attack {
    timer: f32,
    projectiles: Vec<f32>,
}

impl HasEffect for Ability<Attack> {
    fn effect(&self) -> Box<dyn Effect> {
        Attack {
            timer: 0.0,
            projectiles: vec![],
        }
        .into()
    }
}

impl Effect for Attack {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        let mut modifiers = vec![];
        self.projectiles.retain_mut(|timer| {
            *timer += delta;
            if *timer >= 0.5 {
                modifiers.push(ModifierDesc {
                    modifier: Modifier::AffectHP(-myself.attack),
                    target: Target::Enemy,
                    value_kind: ValueKind::Units,
                });
                false
            } else {
                true
            }
        });
        if self.timer >= 1.0 / myself.attack_speed {
            self.timer = 0.0;
            self.projectiles.push(0.0);
            modifiers.push(ModifierDesc {
                modifier: Modifier::NormalAttack,
                target: Target::Enemy,
                value_kind: ValueKind::Units,
            });
        }
        self.timer += delta;
        modifiers
    }
}
