use bevy::prelude::*;

use crate::{hero::HeroId, scene::landing::HeroWatch};

use super::LocalSchedule;

pub struct HomePlugin;

impl Plugin for HomePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (init, update).run_if(any_with_component::<Home>),
        );
    }
}

#[derive(Component)]
pub struct HeroState {
    pub active: bool,
    pub changed: bool,
}

#[derive(Component)]
pub struct Home {}

impl Home {}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &mut Home, &Children), Added<Home>>,
) {
    for (entity, mut home, children) in query.iter_mut() {
        for hero in children.iter() {
            let transform = TransformBundle {
                local: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..Default::default()
            };

            commands.entity(*hero).insert((
                transform,
                HeroState {
                    active: true,
                    changed: true,
                },
                VisibilityBundle {
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
            ));
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
    watch: Res<HeroWatch>,
    query: Query<&Children, With<Home>>,
    hero_ids: Query<&HeroId>,
) {
    for children in query.iter() {
        for entity in children {
            let Ok(hero) = hero_ids.get(*entity) else {
                continue;
            };
            if hero.0 == watch.id {
                commands.entity(*entity).insert(Visibility::Inherited);
            } else {
                commands.entity(*entity).insert(Visibility::Hidden);
            }
        }
    }
}
