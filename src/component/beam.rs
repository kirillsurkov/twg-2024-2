use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

use crate::{
    battle::RoundCapture, battle_bridge::RoundCaptureResource, hero::HeroId,
    scene::landing::HeroWatch,
};

use super::{fight_state::FightState, LocalSchedule};

#[derive(Component)]
pub struct Beam {
    duration: f32,
    timer: f32,
    transform: Transform,
    color: Color,
}

impl Beam {
    pub fn new(duration: f32) -> Self {
        Self {
            duration,
            timer: 0.0,
            transform: Transform::IDENTITY,
            color: Color::CYAN,
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

pub struct BeamPlugin;

impl Plugin for BeamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (
                setup,
                update,
                filter.run_if(resource_exists::<RoundCaptureResource>),
            ),
        );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Beam), Added<Beam>>,
) {
    for (entity, beam) in query.iter() {
        commands.entity(entity).insert((
            PbrBundle {
                mesh: meshes.add(Cylinder {
                    half_height: 100.0,
                    radius: 1.0,
                }),
                material: materials.add(StandardMaterial {
                    base_color: (beam.color * 4.0).with_a(0.1),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    ..Default::default()
                }),
                transform: beam.transform,
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            AudioBundle {
                source: asset_server.load("embedded://teleport.ogg"),
                ..Default::default()
            },
            NotShadowCaster,
            NotShadowReceiver,
        ));
    }
}

fn update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Beam, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut beam, mut transform) in query.iter_mut() {
        if beam.timer >= beam.duration {
            beam.timer = beam.duration;
            commands.entity(entity).despawn_recursive();
        } else {
            beam.timer += time.delta_seconds();
        }

        let factor = (beam.duration - beam.timer) / beam.duration;

        transform.scale.x = factor;
        transform.scale.z = factor;
    }
}

fn filter(
    mut commands: Commands,
    capture: Res<RoundCaptureResource>,
    watch: Res<HeroWatch>,
    query: Query<(Entity, &HeroId), With<Beam>>,
) {
    for (entity, id) in query.iter() {
        let round = capture.by_player(&id.0).unwrap();
        let show = match round {
            RoundCapture::Fight {
                player1, player2, ..
            } => *player1 == watch.id || *player2 == watch.id,
            RoundCapture::Skip(player) => *player == watch.id,
        };
        if show {
            commands.entity(entity).insert(Visibility::Inherited);
        } else {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
}
