use arena::ArenaPlugin;
use beam::BeamPlugin;
use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use complex_anim_player::ComplexAnimPlayerPlugin;
use fight_state::FightStatePlugin;
use game_timer::GameTimerPlugin;
use home::HomePlugin;
use land::LandPlugin;
use projectile::ProjectilePlugin;
use wheel::WheelPlugin;

pub mod arena;
pub mod beam;
pub mod complex_anim_player;
pub mod fight_state;
pub mod game_timer;
pub mod home;
pub mod land;
pub mod model;
pub mod projectile;
pub mod wheel;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct LocalSchedule;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GameTimerPlugin,
            FightStatePlugin,
            ComplexAnimPlayerPlugin,
            BeamPlugin,
            ProjectilePlugin,
            WheelPlugin,
            LandPlugin,
            HomePlugin,
            ArenaPlugin,
        ));
    }
}
