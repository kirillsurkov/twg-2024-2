use bevy::prelude::*;

use crate::{hero::HeroComponent, scene::landing::HeroSelected};

use super::LocalSchedule;

pub struct HomePlugin;

impl Plugin for HomePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, added);
    }
}

#[derive(Component)]
pub struct State {
    pub active: bool,
    pub changed: bool,
}

#[derive(Component)]
pub struct Home {}

impl Home {}

fn added(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &mut Home, &Children), Added<Home>>,
    selected: Option<Res<HeroSelected>>,
    heroes: Query<&HeroComponent>,
) {
    for (entity, mut home, children) in query.iter_mut() {
        let selected = selected.as_ref().unwrap();

        for hero in children.iter() {
            if heroes.get(*hero).unwrap().id != selected.id {
                commands.entity(*hero).despawn_recursive();
            }
        }

        let hero = *children
            .iter()
            .find(|c| heroes.get(**c).unwrap().id == selected.id)
            .unwrap();

        let transform = TransformBundle {
            local: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        };

        commands.entity(hero).insert((
            transform,
            State {
                active: true,
                changed: true,
            },
            VisibilityBundle::default(),
        ));

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
