use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use crate::{
    battle::fight::Owner, battle_bridge::RoundCaptureResource, hero::HeroId,
    scene::landing::HeroWatch,
};

use super::LocalSchedule;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (init, update).run_if(any_with_component::<Arena>),
        );
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

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &mut Arena, &Children), Added<Arena>>,
    capture: Res<RoundCaptureResource>,
    hero_ids: Query<&HeroId>,
    with_parent: Query<&Parent>,
) {
    for (entity, mut arena, children) in query.iter_mut() {
        for capture in &capture.0 {
            for hero in children.iter() {
                let id = &hero_ids.get(*hero).unwrap().0;
                let fighter = if id == capture.player1 {
                    Owner::Fighter1
                } else if id == capture.player2 {
                    Owner::Fighter2
                } else {
                    continue;
                };

                let x = match fighter {
                    Owner::Fighter1 => -4.0,
                    Owner::Fighter2 => 4.0,
                };

                let rotation = match fighter {
                    Owner::Fighter1 => Quat::from_rotation_y(FRAC_PI_2),
                    Owner::Fighter2 => Quat::from_rotation_y(-FRAC_PI_2),
                };

                let transform = TransformBundle {
                    local: Transform::from_translation(Vec3::new(x, 0.0, 0.0))
                        .with_rotation(rotation),
                    ..Default::default()
                };

                let mut parent = commands.entity(with_parent.get(*hero).unwrap().get());
                let mut hero_node = Entity::PLACEHOLDER;
                parent.with_children(|p| {
                    hero_node = p.spawn((transform, VisibilityBundle::default())).id();
                });

                commands
                    .entity(*hero)
                    .insert((
                        HeroState {
                            active: true,
                            changed: true,
                        },
                        TransformBundle::default(),
                        VisibilityBundle {
                            visibility: Visibility::Hidden,
                            ..Default::default()
                        },
                    ))
                    .set_parent(hero_node);
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

fn update(
    mut commands: Commands,
    capture: Res<RoundCaptureResource>,
    watch: Res<HeroWatch>,
    heroes: Query<Entity, With<HeroState>>,
    hero_ids: Query<&HeroId>,
) {
    let capture = capture.by_player(&watch.id).unwrap();
    for entity in heroes.iter() {
        let Ok(hero) = hero_ids.get(entity) else {
            continue;
        };
        if hero.0 == capture.player1 || hero.0 == capture.player2 {
            commands.entity(entity).insert(Visibility::Inherited);
        } else {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
}
