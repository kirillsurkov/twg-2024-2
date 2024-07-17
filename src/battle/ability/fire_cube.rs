use crate::battle::{
    effect::{Effect, HasEffect},
    fight::Fighter,
    modifier::{Modifier, ModifierDesc, Target, ValueKind},
};

use super::Ability;

pub const CUBE_SIDE: u32 = 3;
pub const CUBE_FIRES: u32 = CUBE_SIDE * CUBE_SIDE * CUBE_SIDE;

#[derive(Debug)]
pub struct FireCube {
    current: u32,
    fires_charging: Vec<(u32, f32)>,
    fires_ready: Vec<(u32, f32)>,
    projectiles: Vec<f32>,
}

impl HasEffect for Ability<FireCube> {
    fn effect(&self) -> Box<dyn Effect> {
        FireCube {
            current: 0,
            fires_charging: vec![],
            fires_ready: vec![],
            projectiles: vec![],
        }
        .into()
    }
}

impl Effect for FireCube {
    fn update(&mut self, delta: f32, myself: &Fighter, enemy: &Fighter) -> Vec<ModifierDesc> {
        let mut modifiers = vec![];

        self.projectiles.retain_mut(|timer| {
            *timer += delta;
            if *timer >= 0.5 {
                modifiers.push(ModifierDesc {
                    modifier: Modifier::AffectHP(-300.0 / CUBE_FIRES as f32),
                    target: Target::Enemy,
                    value_kind: ValueKind::Ulti,
                });
                false
            } else {
                true
            }
        });

        self.fires_ready.retain_mut(|(i, timer)| {
            *timer += delta;
            if *timer >= (*i + 1) as f32 / CUBE_FIRES as f32 {
                self.projectiles.push(0.0);
                modifiers.push(ModifierDesc {
                    modifier: Modifier::ShootFireCube(*i),
                    target: Target::Enemy,
                    value_kind: ValueKind::Units,
                });
                false
            } else {
                true
            }
        });

        for _ in self.fires_charging.len()..(myself.mana * CUBE_FIRES as f32 / 100.0) as usize {
            modifiers.push(ModifierDesc {
                modifier: Modifier::SpawnFireCube(self.current),
                target: Target::Myself,
                value_kind: ValueKind::Units,
            });
            self.fires_charging.push((self.current, 0.0));
            self.current += 1;
        }

        if myself.mana >= 100.0 {
            self.fires_ready.append(&mut self.fires_charging);
            modifiers.extend(vec![
                ModifierDesc {
                    modifier: Modifier::AffectMana(-100.0),
                    target: Target::Myself,
                    value_kind: ValueKind::Units,
                },
                ModifierDesc {
                    modifier: Modifier::Ulti,
                    target: Target::Myself,
                    value_kind: ValueKind::Units,
                },
            ]);
        }

        modifiers
    }
}
