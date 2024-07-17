use avatar::AvatarPlugin;
use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use cards::CardsPlugin;
use fight_arena_layout::FightArenaLayout;
use fight_home_layout::FightHomeLayout;
use game_timer::GameTimerPlugin;
use hp_mana_bars::HpManaBarsPlugin;
use layout::LayoutPlugin;
use players::PlayersPlugin;
use screen::ScreenPlugin;
use stats::StatsPlugin;

pub mod fight_arena_layout;
pub mod fight_home_layout;

mod avatar;
mod cards;
mod game_timer;
mod hp_mana_bars;
mod layout;
mod players;
mod screen;
mod stats;

const DCOLOR: BackgroundColor = BackgroundColor(Color::rgba(0.0, 0.0, 1.0, 0.1));

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
            StatsPlugin,
            AvatarPlugin,
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

#[derive(Component, PartialEq)]
enum ClickState {
    None,
    Hovered,
    Pressed,
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiAssets {
        font_comic: asset_server.load("embedded://comic.ttf"),
    });
}
