use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

use crate::{hero::HeroComponent, scene::landing::HeroSelected};

use super::LocalSchedule;

pub struct LandPlugin;

impl Plugin for LandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (added, show.run_if(resource_exists::<HeroSelected>)),
        );
    }
}

#[derive(Component)]
pub struct HeroState;

#[derive(Component)]
pub struct Land {
    index: usize,
    timer: f32,
    ready: bool,
}

impl Land {
    pub fn new() -> Self {
        Self {
            index: 0,
            timer: 0.0,
            ready: false,
        }
    }
}

impl Land {
    pub fn ready(&self) -> bool {
        self.ready
    }
}

#[derive(Component)]
struct Beam {
    timer: f32,
}

fn added(
    mut commands: Commands,
    mut query: Query<(Entity, &Children), Added<Land>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    selected: Option<Res<HeroSelected>>,
    heroes: Query<&HeroComponent>,
) {
    for (entity, children) in query.iter_mut() {
        let selected = selected.as_ref().unwrap();

        let mut children = children.iter().map(|e| *e).collect::<Vec<_>>();
        children.sort_unstable_by_key(|c| heroes.get(*c).unwrap().id != selected.id);

        for (i, child) in children.into_iter().enumerate() {
            let (x, y) = match i {
                0 => (0.0, 0.0),
                1 => (-2.5, -5.0),
                2 => (2.5, -5.0),
                3 => (-5.0, -10.0),
                4 => (0.0, -10.0),
                5 => (5.0, -10.0),
                _ => unreachable!(),
            };

            let transform = TransformBundle {
                local: Transform::from_translation(Vec3::new(x, 0.0, y)),
                ..Default::default()
            };
            commands
                .entity(child)
                .insert((
                    transform,
                    HeroState,
                    VisibilityBundle {
                        visibility: Visibility::Hidden,
                        ..Default::default()
                    },
                ))
                .with_children(|p| {
                    p.spawn((
                        PbrBundle {
                            mesh: meshes.add(Cylinder {
                                half_height: 100.0,
                                radius: 1.0,
                            }),
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgba(0.0, 4.0, 4.0, 0.1),
                                alpha_mode: AlphaMode::Blend,
                                unlit: true,
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                        Beam { timer: 0.0 },
                        NotShadowCaster,
                        NotShadowReceiver,
                    ));
                });
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

fn show(
    mut commands: Commands,
    mut query: Query<(&mut Land, &Children)>,
    mut visibilities: Query<&mut Visibility>,
    mut beams: Query<(Entity, &mut Beam, &mut Transform, &Parent)>,
    asset_server: Res<AssetServer>,
    selected: Res<HeroSelected>,
    heroes: Query<&HeroComponent>,
    time: Res<Time>,
) {
    for (mut land, children) in query.iter_mut() {
        let mut children = children
            .iter()
            .filter_map(|e| heroes.get(*e).ok().map(|h| (*e, h)))
            .collect::<Vec<_>>();
        children.sort_by_key(|(_, h)| h.id == selected.id);

        land.ready = land.index == children.len();

        if land.timer >= 0.25 {
            if land.index < children.len() {
                *visibilities.get_mut(children[land.index].0).unwrap() = Visibility::Visible;
                land.timer = 0.0;
                land.index += 1;
                commands.spawn(AudioBundle {
                    source: asset_server.load("embedded://teleport.ogg"),
                    ..Default::default()
                });
            } else {
                land.ready = true;
            }
        } else {
            land.timer += time.delta_seconds();
        }

        for (i, (child, _)) in children.into_iter().enumerate() {
            if i >= land.index {
                continue;
            }

            *visibilities.get_mut(child).unwrap() = Visibility::Visible;

            let (entity, mut beam, mut transform, _) = beams
                .iter_mut()
                .find(|(_, _, _, p)| p.get() == child)
                .unwrap();

            if beam.timer >= 0.5 {
                beam.timer = 0.5;
                *visibilities.get_mut(entity).unwrap() = Visibility::Hidden;
            } else {
                beam.timer += time.delta_seconds();
            }

            transform.scale.x = (0.5 - beam.timer) * 2.0;
            transform.scale.y = (0.5 - beam.timer) * 2.0;
        }
    }
}
