use bevy::prelude::*;

use super::LocalSchedule;

#[derive(Resource)]
pub struct GameTimer {
    pub value: f32,
    pub max: f32,
    pub red: bool,
    pub fired: bool,
}

impl GameTimer {
    pub fn restart(&mut self, max: f32, red: bool) {
        self.value = 0.0;
        self.max = max + 1.0;
        self.red = red;
        self.fired = false;
    }
}

pub struct GameTimerPlugin;

impl Plugin for GameTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, update);
        app.insert_resource(GameTimer {
            value: 0.0,
            max: 0.0,
            red: false,
            fired: false,
        });
    }
}

fn update(mut timer: ResMut<GameTimer>, time: Res<Time>) {
    if !timer.fired {
        if timer.value >= timer.max {
            return;
        }
        timer.value += time.delta_seconds();
        if timer.value >= timer.max {
            timer.value = timer.max;
            timer.fired = true;
        }
    }
}
