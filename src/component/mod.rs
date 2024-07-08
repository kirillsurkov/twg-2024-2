use arena::ArenaPlugin;
use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use complex_anim_player::ComplexAnimPlayerPlugin;
use home::HomePlugin;
use land::LandPlugin;
use wheel::WheelPlugin;

pub mod arena;
pub mod complex_anim_player;
pub mod home;
pub mod land;
pub mod model;
pub mod wheel;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct LocalSchedule;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WheelPlugin,
            ComplexAnimPlayerPlugin,
            LandPlugin,
            HomePlugin,
            ArenaPlugin,
        ));
    }
}
