use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use cards::CardsPlugin;
use fight_arena_layout::FightArenaLayout;
use fight_home_layout::FightHomeLayout;
use game_timer::GameTimerPlugin;
use hp_mana_bars::HpManaBarsPlugin;
use layout::LayoutPlugin;
use players::PlayersPlugin;
use screen::ScreenPlugin;

use crate::battle::fight;

pub mod cards;
pub mod fight_arena_layout;
pub mod fight_home_layout;
pub mod game_timer;
pub mod hp_mana_bars;
pub mod layout;
pub mod players;
pub mod screen;

const DCOLOR: BackgroundColor = BackgroundColor(Color::rgba(0.0, 0.0, 1.0, 0.1));

#[derive(Resource)]
pub struct FightState {
    pub current: fight::State,
}

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct LocalSchedule;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LayoutPlugin,
            ScreenPlugin,
            GameTimerPlugin,
            PlayersPlugin,
            CardsPlugin,
            HpManaBarsPlugin,
            FightHomeLayout,
            FightArenaLayout,
        ));
        app.add_systems(Startup, init);
    }
}

#[derive(Resource)]
struct UiAssets {
    font_comic: Handle<Font>,
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiAssets {
        font_comic: asset_server.load("comic.ttf"),
    });
}
