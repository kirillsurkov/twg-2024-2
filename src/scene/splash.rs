use bevy::prelude::*;

use super::{GameState, LocalSchedule, Root};

#[derive(Resource)]
struct State {
    timer: f32,
}

pub struct Splash;

impl Plugin for Splash {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, update.run_if(in_state(GameState::Splash)));
    }
}

fn update(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    state: Option<ResMut<State>>,
    time: Res<Time>,
    query: Query<Entity, Added<Root>>,
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
