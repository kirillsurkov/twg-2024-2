use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

use crate::{component::arena::Arena, hero::HeroesRoot};

use super::{landing::HeroSelected, GameState, InitScene, Root};

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
    query: Query<Entity, (With<Root>, Added<InitScene>)>,
) {
    for root in query.iter() {
        println!("FIGHT ARENA INIT FOR {}", selected.name);
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

            p.spawn((Arena {}, HeroesRoot));
        });
        return;
    }

    let mut state = state.unwrap();

    state.timer += time.delta_seconds();
    if state.timer >= 3.0 {
        next_state.set(GameState::FightHome);
    }
}
