use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::Ability;

#[derive(Debug)]
pub struct StarWars {
    current: u32,
    swiborgs_charging: Vec<(u32, f32)>,
    swiborgs_ready: Vec<(u32, f32)>,
    projectiles: Vec<f32>,
}

impl HasEffect for Ability<StarWars> {
    fn effect(&self) -> Box<dyn Effect> {
        StarWars {
            current: 0,
            swiborgs_charging: vec![],
            swiborgs_ready: vec![],
            projectiles: vec![],
        }
        .into()
    }
}

impl Effect for StarWars {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        let mut modifiers = vec![];

        self.projectiles.retain_mut(|timer| {
            *timer += delta;
            if *timer >= 0.5 {
                modifiers.push(ModifierDesc {
                    modifier: Modifier::AffectHP(-60.0),
                    target: Target::Enemy,
                    value_kind: ValueKind::Units,
                });
                false
            } else {
                true
            }
        });

        self.swiborgs_ready.retain_mut(|(i, timer)| {
            *timer += delta;
            if *timer >= 0.1 * (*i + 1) as f32 {
                self.projectiles.push(0.0);
                modifiers.push(ModifierDesc {
                    modifier: Modifier::ShootSwiborg(*i),
                    target: Target::Enemy,
                    value_kind: ValueKind::Units,
                });
                false
            } else {
                true
            }
        });

        if myself.mana >= (self.swiborgs_charging.len() + 1) as f32 * 20.0 {
            modifiers.push(ModifierDesc {
                modifier: Modifier::SpawnSwiborg(self.current),
                target: Target::Myself,
                value_kind: ValueKind::Units,
            });
            self.swiborgs_charging.push((self.current, 0.0));
            self.current += 1;
        }

        if myself.mana >= 100.0 {
            self.swiborgs_ready.append(&mut self.swiborgs_charging);
            modifiers.push(ModifierDesc {
                modifier: Modifier::AffectMana(-100.0),
                target: Target::Myself,
                value_kind: ValueKind::Units,
            });
        }

        modifiers
    }
}
