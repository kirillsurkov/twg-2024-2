use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

use crate::{hero::HeroId, scene::landing::HeroSelected};

use super::{beam::Beam, LocalSchedule};

pub struct LandPlugin;

impl Plugin for LandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (added, show.run_if(resource_exists::<HeroSelected>))
                .run_if(any_with_component::<Land>),
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

fn added(
    mut commands: Commands,
    mut query: Query<(Entity, &Children), Added<Land>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    selected: Res<HeroSelected>,
    hero_ids: Query<&HeroId>,
) {
    for (entity, children) in query.iter_mut() {
        let mut children = children.iter().map(|e| *e).collect::<Vec<_>>();
        children.sort_unstable_by_key(|c| hero_ids.get(*c).unwrap().0 != selected.id);

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
            commands.entity(child).insert((
                transform,
                HeroState,
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

fn show(
    mut commands: Commands,
    mut query: Query<(&mut Land, &Children)>,
    selected: Res<HeroSelected>,
    hero_ids: Query<&HeroId>,
    time: Res<Time>,
) {
    for (mut land, children) in query.iter_mut() {
        let mut children = children
            .iter()
            .filter_map(|e| hero_ids.get(*e).ok().map(|h| (*e, h)))
            .collect::<Vec<_>>();
        children.sort_by_key(|(_, h)| h.0 == selected.id);

        land.ready = land.index == children.len();

        if land.timer >= 0.25 {
            if land.index < children.len() {
                commands
                    .entity(children[land.index].0)
                    .with_children(|p| {
                        p.spawn(Beam::new(0.5));
                    })
                    .insert(Visibility::Inherited);
                land.timer = 0.0;
                land.index += 1;
            } else {
                land.ready = true;
            }
        } else {
            land.timer += time.delta_seconds();
        }
    }
}
