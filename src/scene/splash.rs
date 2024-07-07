use bevy::prelude::*;

use super::{GameState, InitScene, Root};

#[derive(Resource)]
pub struct State {
    timer: f32,
}

pub fn update(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    state: Option<ResMut<State>>,
    time: Res<Time>,
    query: Query<Entity, (With<Root>, Added<InitScene>)>,
) {
    for root in query.iter() {
        println!("SPLASH INIT");
        commands.insert_resource(State { timer: 0.0 });
        return;
    }

    let mut state = state.unwrap();

    state.timer += time.delta_seconds();
    if state.timer >= 2.0 {
        next_state.set(GameState::SelectHero);
    }
}
