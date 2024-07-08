use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

use crate::{component::land::Land, hero::HeroesRoot};

use super::{GameState, Root};

#[derive(Resource)]
pub struct HeroSelected {
    pub id: String,
}

#[derive(Resource)]
pub struct State {
    timer: f32,
}

pub fn update(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    selected: Res<HeroSelected>,
    state: Option<ResMut<State>>,
    time: Res<Time>,
    query: Query<Entity, Added<Root>>,
    land: Query<&Land>,
) {
    for root in query.iter() {
        println!("LANDING INIT FOR {}", selected.id);
        commands.insert_resource(State { timer: 0.0 });
        commands.entity(root).with_children(|p| {
            p.spawn((
                Camera3dBundle {
                    camera: Camera {
                        hdr: true,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(-2.0, 10.0, 10.0))
                        .looking_at(Vec3::new(0.0, 0.0, -5.0), Vec3::Y),
                    ..default()
                },
                BloomSettings::default(),
            ));

            p.spawn(DirectionalLightBundle {
                directional_light: DirectionalLight {
                    color: Color::rgb(0.98, 0.95, 0.82),
                    shadows_enabled: true,
                    illuminance: 1000.0,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .looking_at(Vec3::new(0.15, -0.15, -0.25), Vec3::Y),
                ..Default::default()
            });

            p.spawn((Land::default(), HeroesRoot));
        });
        return;
    }

    let mut state = state.unwrap();

    let land = land.single();

    if land.ready() {
        state.timer += time.delta_seconds();
        if state.timer >= 3.0 {
            next_state.set(GameState::FightHome);
        }
    }
}
