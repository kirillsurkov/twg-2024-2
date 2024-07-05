use std::f32::consts::PI;

use bevy::prelude::*;

pub struct WheelPlugin;

impl Plugin for WheelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (added, scroll));
    }
}

#[derive(Component)]
pub struct Active;

#[derive(Component)]
pub struct Wheel {
    radius: f32,
    current: usize,
    max: usize,
}

impl Wheel {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            current: 0,
            max: 0,
        }
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
            commands
                .entity(*child)
                .insert((transform, VisibilityBundle::default()));
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
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Children, &mut Wheel, &mut Transform)>,
    time: Res<Time>,
) {
    let Ok((children, mut scroll, mut transform)) = query.get_single_mut() else {
        return;
    };

    let left = keyboard_input.just_pressed(KeyCode::ArrowLeft);
    let right = keyboard_input.just_pressed(KeyCode::ArrowRight);

    if left || right {
        commands
            .entity(*children.get(scroll.current as usize).unwrap())
            .remove::<Active>();
        if left {
            scroll.current = (scroll.current + scroll.max - 1) % scroll.max;
        }
        if right {
            scroll.current = (scroll.current + 1) % scroll.max;
        }
        commands
            .entity(*children.get(scroll.current as usize).unwrap())
            .insert(Active);
    }

    let ang = -2.0 * PI * scroll.current as f32 / scroll.max as f32;

    transform.rotation = transform
        .rotation
        .slerp(Quat::from_rotation_y(ang), 10.0 * time.delta_seconds());
}
