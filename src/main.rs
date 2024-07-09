use bevy::{app::MainScheduleOrder, prelude::*};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_hanabi::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_round_ui::prelude::BevyRoundUiDefaultPlugins;
use component::ComponentsPlugin;
use hero::HeroesPlugin;
use scene::ScenesPlugin;

mod battle;
mod component;
mod hero;
mod scene;

fn main() {
    let mut app = App::new();

    app.add_schedule(Schedule::new(hero::LocalSchedule))
        .add_plugins(HeroesPlugin)
        .world
        .resource_mut::<MainScheduleOrder>()
        .insert_after(Update, hero::LocalSchedule);

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

    app.add_plugins(DefaultPlugins)
        .add_plugins(HanabiPlugin)
        .add_plugins(BevyRoundUiDefaultPlugins)
        .add_plugins(EmbeddedAssetPlugin::default())
        //.add_plugins(NoCameraPlayerPlugin)
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // commands.spawn((
    //     Camera2dBundle {
    //         camera: Camera {
    //             order: 2,
    //             ..Default::default()
    //         },
    //         camera_2d: Camera2d {},
    //         ..Default::default()
    //     },
    //     IsDefaultUiCamera,
    // ));
}
