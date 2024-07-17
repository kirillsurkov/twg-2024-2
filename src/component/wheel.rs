use std::f32::consts::PI;

use bevy::{audio::Volume, prelude::*};

use crate::MASTER_VOLUME;

use super::LocalSchedule;

pub struct WheelPlugin;

impl Plugin for WheelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, (added, scroll));
    }
}

#[derive(Component)]
pub struct HeroState {
    pub active: bool,
    pub changed: bool,
}

#[derive(Component)]
pub struct Wheel {
    radius: f32,
    current: usize,
    max: usize,
    selected: bool,
    changed: bool,
}

impl Wheel {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            current: 0,
            max: 0,
            selected: false,
            changed: true,
        }
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn changed(&self) -> bool {
        self.changed
    }
}

fn added(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Wheel, &Children), Added<Wheel>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut wheel, children) in query.iter_mut() {
        wheel.max = children.len();
        for (i, child) in children.iter().enumerate() {
            let ang = 2.0 * PI * i as f32 / wheel.max as f32;
            let x = ang.sin() * wheel.radius;
            let y = ang.cos() * wheel.radius;
            let transform = TransformBundle {
                local: Transform::from_translation(Vec3::new(x, 0.0, y))
                    .with_rotation(Quat::from_rotation_y(ang)),
                ..Default::default()
            };
            commands
                .entity(*child)
                .insert((
                    transform,
                    HeroState {
                        active: i == 0,
                        changed: true,
                    },
                    VisibilityBundle::default(),
                ))
                .with_children(|p| {
                    p.spawn(PbrBundle {
                        mesh: meshes.add(Cylinder {
                            radius: 2.5,
                            half_height: 0.05,
                        }),
                        material: materials.add(StandardMaterial {
                            base_color: Color::GRAY,
                            ..Default::default()
                        }),
                        transform: Transform::from_translation(Vec3::new(0.0, -0.05, 0.0)),
                        ..Default::default()
                    });
                    p.spawn(PbrBundle {
                        mesh: meshes.add(Cylinder {
                            radius: 0.5,
                            half_height: 0.45,
                        }),
                        material: materials.add(StandardMaterial {
                            base_color: Color::GRAY,
                            ..Default::default()
                        }),
                        transform: Transform::from_translation(Vec3::new(0.0, -0.55, 0.0)),
                        ..Default::default()
                    });
                    p.spawn(PbrBundle {
                        mesh: meshes.add(Torus {
                            major_radius: 2.49,
                            minor_radius: 0.01,
                        }),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgb(0.0, 0.0, 4.0),
                            unlit: true,
                            ..Default::default()
                        }),
                        // transform: Transform::from_translation(Vec3::new(0.0, -0.05, 0.0)),
                        ..Default::default()
                    });
                });
        }

        commands
            .entity(entity)
            .insert((
                TransformBundle {
                    local: Transform {
                        translation: Vec3::new(0.0, 0.0, -wheel.radius),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                VisibilityBundle::default(),
            ))
            .with_children(|p| {
                p.spawn(PbrBundle {
                    mesh: meshes.add(Torus {
                        major_radius: wheel.radius,
                        minor_radius: 0.01,
                    }),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(0.0, 0.0, 4.0),
                        unlit: true,
                        ..Default::default()
                    }),
                    transform: Transform::from_translation(Vec3::new(0.0, -1.0, 0.0)),
                    ..Default::default()
                });
            });
    }
}

fn scroll(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Wheel, &mut Transform, &Children)>,
    mut states: Query<&mut HeroState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for (entity, mut wheel, mut transform, children) in query.iter_mut() {
        wheel.changed = false;

        for child in children {
            if let Ok(mut child) = states.get_mut(*child) {
                child.changed = false;
            }
        }

        if wheel.selected {
            continue;
        }

        wheel.selected = keyboard_input.just_pressed(KeyCode::Enter);

        let left = keyboard_input.just_pressed(KeyCode::ArrowLeft);
        let right = keyboard_input.just_pressed(KeyCode::ArrowRight);

        if left || right {
            wheel.changed = true;
            let mut state = states
                .get_mut(*children.get(wheel.current).unwrap())
                .unwrap();
            state.changed = true;
            state.active = false;
            if left {
                wheel.current = (wheel.current + wheel.max - 1) % wheel.max;
            }
            if right {
                wheel.current = (wheel.current + 1) % wheel.max;
            }
            let mut state = states
                .get_mut(*children.get(wheel.current).unwrap())
                .unwrap();
            state.changed = true;
            state.active = true;

            commands.entity(entity).with_children(|p| {
                p.spawn(AudioBundle {
                    source: asset_server.load("embedded://scroll.ogg"),
                    settings: PlaybackSettings {
                        volume: Volume::new(MASTER_VOLUME),
                        ..Default::default()
                    },
                });
            });
        }

        let ang = -2.0 * PI * wheel.current as f32 / wheel.max as f32;

        transform.rotation = transform
            .rotation
            .slerp(Quat::from_rotation_y(ang), 10.0 * time.delta_seconds());
    }
}
