use bevy::{
    app::MainScheduleOrder,
    audio::{PlaybackMode, Volume},
    prelude::*,
    window::{WindowMode, WindowResolution},
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_hanabi::prelude::*;
use component::ComponentsPlugin;
use hero::HeroesPlugin;
use iyes_perf_ui::prelude::*;
use scene::ScenesPlugin;
use ui::UIPlugin;

mod battle;
mod battle_bridge;
mod component;
mod hero;
mod scene;
mod ui;

pub const MASTER_VOLUME: f32 = 0.1;
pub const BUTTON_VOLUME: f32 = 0.05;

fn main() {
    let mut app = App::new();

    app.add_schedule(Schedule::new(ui::LocalSchedule))
        .add_plugins(UIPlugin)
        .world
        .resource_mut::<MainScheduleOrder>()
        .insert_after(Update, ui::LocalSchedule);

    app.add_schedule(Schedule::new(hero::LocalSchedule))
        .add_plugins(HeroesPlugin)
        .world
        .resource_mut::<MainScheduleOrder>()
        .insert_after(ui::LocalSchedule, hero::LocalSchedule);

    app.add_schedule(Schedule::new(component::LocalSchedule))
        .add_plugins(ComponentsPlugin)
        .world
        .resource_mut::<MainScheduleOrder>()
        .insert_after(hero::LocalSchedule, component::LocalSchedule);

    app.add_schedule(Schedule::new(scene::LocalSchedule))
        .add_plugins(ScenesPlugin)
        .world
        .resource_mut::<MainScheduleOrder>()
        .insert_after(component::LocalSchedule, scene::LocalSchedule);

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "TWG: UNITED (ВСЕ В СБОРЕ)".to_string(),
            // resolution: WindowResolution::new(1600.0, 900.0),
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        }),
        ..Default::default()
    }))
    .add_plugins(HanabiPlugin)
    .add_plugins(EmbeddedAssetPlugin::default())
    .add_plugins((PerfUiPlugin, bevy::diagnostic::FrameTimeDiagnosticsPlugin))
    // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
    .add_systems(Update, bevy::window::close_on_esc)
    .add_systems(Startup, init)
    .run();
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        PerfUiRoot {
            display_labels: false,
            layout_horizontal: true,
            ..Default::default()
        },
        PerfUiEntryFPS::default(),
    ));
    commands.spawn(AudioBundle {
        source: asset_server.load("embedded://main_theme.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(MASTER_VOLUME * 0.8),
            ..Default::default()
        },
    });
}
