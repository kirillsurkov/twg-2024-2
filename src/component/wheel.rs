use std::f32::consts::PI;

use bevy::prelude::*;

use super::LocalSchedule;

pub struct WheelPlugin;

impl Plugin for WheelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, (added, scroll));
    }
}

#[derive(Component)]
pub struct State {
    pub active: bool,
    pub changed: bool,
}

#[derive(Component)]
pub struct Wheel {
    radius: f32,
    current: usize,
    max: usize,
    selected: bool,
}

impl Wheel {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            current: 0,
            max: 0,
            selected: false,
        }
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn selected(&self) -> bool {
        self.selected
    }
}

fn added(mut commands: Commands, mut query: Query<(Entity, &mut Wheel, &Children), Added<Wheel>>) {
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
            commands.entity(*child).insert((
                transform,
                State {
                    active: i == 0,
                    changed: true,
                },
                VisibilityBundle::default(),
            ));
        }
        commands.entity(entity).insert((
            TransformBundle {
                local: Transform {
                    translation: Vec3::new(0.0, 0.0, -wheel.radius),
                    ..Default::default()
                },
                ..Default::default()
            },
            VisibilityBundle::default(),
        ));
    }
}

fn scroll(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Children, &mut Wheel, &mut Transform)>,
    mut states: Query<&mut State>,
    time: Res<Time>,
) {
    for (children, mut wheel, mut transform) in query.iter_mut() {
        for child in children {
            states.get_mut(*child).unwrap().changed = false;
        }

        if wheel.selected {
            continue;
        }

        wheel.selected = keyboard_input.just_pressed(KeyCode::Enter);

        let left = keyboard_input.just_pressed(KeyCode::ArrowLeft);
        let right = keyboard_input.just_pressed(KeyCode::ArrowRight);

        if left || right {
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
        }

        let ang = -2.0 * PI * wheel.current as f32 / wheel.max as f32;

        transform.rotation = transform
            .rotation
            .slerp(Quat::from_rotation_y(ang), 10.0 * time.delta_seconds());
    }
}
