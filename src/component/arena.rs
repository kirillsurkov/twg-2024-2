use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use crate::{battle_bridge::RoundCaptureResource, hero::HeroId, scene::landing::HeroSelected};

use super::LocalSchedule;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, added.run_if(any_with_component::<Arena>));
    }
}

#[derive(Component)]
pub struct HeroState {
    pub active: bool,
    pub changed: bool,
}

#[derive(Component)]
pub struct Arena {}

impl Arena {}

fn added(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &mut Arena, &Children), Added<Arena>>,
    selected: Res<HeroSelected>,
    capture: Res<RoundCaptureResource>,
    hero_ids: Query<&HeroId>,
) {
    for (entity, mut arena, children) in query.iter_mut() {
        let capture = capture.by_player(&selected.id).unwrap();
        println!("{} vs {}", capture.player1, capture.player2);

        for hero in children.iter() {
            let id = &hero_ids.get(*hero).unwrap().0;
            let fighter = if id == capture.player1 {
                1
            } else if id == capture.player2 {
                2
            } else {
                0
            };
            if fighter == 0 {
                commands.entity(*hero).despawn_recursive();
            } else {
                let x = match fighter {
                    1 => -5.0,
                    2 => 5.0,
                    _ => unreachable!(),
                };

                let rotation = match fighter {
                    1 => Quat::from_rotation_y(FRAC_PI_2),
                    2 => Quat::from_rotation_y(-FRAC_PI_2),
                    _ => unreachable!(),
                };

                let transform = TransformBundle {
                    local: Transform::from_translation(Vec3::new(x, 0.0, 0.0))
                        .with_rotation(rotation),
                    ..Default::default()
                };

                commands.entity(*hero).insert((
                    transform,
                    HeroState {
                        active: true,
                        changed: true,
                    },
                    VisibilityBundle::default(),
                ));
            }
        }

        commands
            .entity(entity)
            .insert((
                TransformBundle {
                    local: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                VisibilityBundle::default(),
            ))
            .with_children(|p| {
                p.spawn(PbrBundle {
                    mesh: meshes.add(Plane3d {
                        normal: Direction3d::Y,
                    }),
                    material: materials.add(StandardMaterial::default()),
                    transform: Transform::from_scale(Vec3::splat(10000.0)),
                    ..Default::default()
                });
            });
    }
}
