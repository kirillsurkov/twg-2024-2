use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use cards::CardsPlugin;
use fight_home_layout::FightHomeLayout;
use players::PlayersPlugin;
use screen::ScreenPlugin;

pub mod cards;
pub mod fight_home_layout;
pub mod players;
pub mod screen;

const DCOLOR: BackgroundColor = BackgroundColor(Color::rgba(0.0, 0.0, 1.0, 0.1));

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct LocalSchedule;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ScreenPlugin, PlayersPlugin, CardsPlugin, FightHomeLayout));
    }
}
