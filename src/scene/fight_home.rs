use std::error::Error;

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_round_ui::prelude::SuperellipseUiMaterial;

use crate::{
    battle_bridge::{BattleResource, RoundCaptureResource},
    component::home::Home,
    hero::HeroesRoot,
    scene::UiRoot,
    ui::fight_home_layout::FightHomeLayout,
};

use super::{landing::HeroSelected, GameState, LocalSchedule, Root};

#[derive(Resource)]
struct State {
    timer: f32,
}

pub struct FightHome;

impl Plugin for FightHome {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (init.map(drop), update.run_if(resource_exists::<State>))
                .run_if(in_state(GameState::FightHome)),
        );
    }
}

fn init(
    mut commands: Commands,
    mut ui_materials: ResMut<Assets<SuperellipseUiMaterial>>,
    selected: Res<HeroSelected>,
    root: Query<Entity, Added<Root>>,
    asset_server: Res<AssetServer>,
) -> Result<(), Box<dyn Error>> {
    let root = root.get_single()?;
    println!("FIGHT HOME INIT FOR {}", selected.id);
    commands.insert_resource(State { timer: 0.0 });
    commands.entity(root).with_children(|p| {
        p.spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 5.0, 9.0))
                    .looking_at(Vec3::new(0.0, 2.0, 4.0), Vec3::Y),
                ..default()
            },
            BloomSettings::default(),
        ));

        p.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::rgb(0.98, 0.95, 0.82),
                shadows_enabled: true,
                illuminance: 1000.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .looking_at(Vec3::new(0.15, -0.15, -0.25), Vec3::Y),
            ..Default::default()
        });

        p.spawn((Home {}, HeroesRoot));

        // p.spawn(AudioBundle {
        //     source: asset_server.load("embedded://wild_darkness.ogg"),
        //     ..Default::default()
        // });
    });

    let font: Handle<Font> = asset_server.load("embedded://comic.ttf");

    let text_card = TextStyle {
        font: font.clone(),
        font_size: 25.0,
        ..Default::default()
    };

    let cards_holder = || NodeBundle {
        style: Style {
            margin: UiRect::new(
                Val::Percent(20.0),
                Val::Px(0.0),
                Val::Px(0.0),
                Val::Percent(5.0),
            ),
            width: Val::Percent(50.0),
            ..Default::default()
        },
        ..Default::default()
    };

    let card_holder = || NodeBundle {
        style: Style {
            // margin: UiRect::all(Val::Percent(5.0)),
            padding: UiRect::horizontal(Val::Percent(5.0)),
            width: Val::Percent(100.0),
            height: Val::Vh(40.0),
            justify_content: JustifyContent::Center,
            align_self: AlignSelf::FlexEnd,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        // background_color: Color::RED.into(),
        ..Default::default()
    };

    let card_header =
        |ui_materials: &mut ResMut<Assets<SuperellipseUiMaterial>>| MaterialNodeBundle {
            material: ui_materials.add(SuperellipseUiMaterial {
                background_color: Color::BLACK,
                border_radius: Vec4::splat(25.0),
                border_color: Color::WHITE,
                border_thickness: 2.0,
            }),
            style: Style {
                margin: UiRect::bottom(Val::Percent(5.0)),
                padding: UiRect::all(Val::Px(25.0)),
                width: Val::Percent(100.0),
                height: Val::Percent(80.0),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        };

    let card_footer =
        |ui_materials: &mut ResMut<Assets<SuperellipseUiMaterial>>| MaterialNodeBundle {
            material: ui_materials.add(SuperellipseUiMaterial {
                background_color: Color::BLACK,
                border_radius: Vec4::splat(25.0),
                border_color: Color::WHITE,
                border_thickness: 2.0,
            }),
            style: Style {
                padding: UiRect::all(Val::Px(5.0)),
                width: Val::Percent(100.0),
                height: Val::Percent(20.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        };

    let info_holder = || NodeBundle {
        style: Style {
            margin: UiRect::new(
                Val::Percent(0.0),
                Val::Px(0.0),
                Val::Px(0.0),
                Val::Percent(1.0),
            ),
            padding: UiRect::horizontal(Val::Percent(5.0)),
            width: Val::Percent(30.0),
            height: Val::Percent(90.0),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        // background_color: Color::CYAN.into(),
        ..Default::default()
    };

    let info_header =
        |ui_materials: &mut ResMut<Assets<SuperellipseUiMaterial>>| MaterialNodeBundle {
            material: ui_materials.add(SuperellipseUiMaterial {
                background_color: Color::BLACK,
                border_radius: Vec4::splat(25.0),
                border_color: Color::WHITE,
                border_thickness: 2.0,
            }),
            style: Style {
                margin: UiRect::bottom(Val::Percent(10.0)),
                padding: UiRect::all(Val::Px(25.0)),
                width: Val::Percent(100.0),
                height: Val::Percent(85.0),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        };

    let info_footer =
        |ui_materials: &mut ResMut<Assets<SuperellipseUiMaterial>>| MaterialNodeBundle {
            material: ui_materials.add(SuperellipseUiMaterial {
                background_color: Color::BLACK,
                border_radius: Vec4::splat(25.0),
                border_color: Color::WHITE,
                border_thickness: 2.0,
            }),
            style: Style {
                // margin: UiRect::bottom(Val::Percent(5.0)),
                padding: UiRect::all(Val::Px(25.0)),
                width: Val::Percent(100.0),
                height: Val::Percent(15.0),
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            ..Default::default()
        };

    commands.spawn((UiRoot, FightHomeLayout));

    Ok(())
}

fn update(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut state: ResMut<State>,
    mut battle: ResMut<BattleResource>,
    time: Res<Time>,
) {
    state.timer += time.delta_seconds();
    if state.timer >= 3.0 {
        commands.insert_resource(RoundCaptureResource(battle.round()));
        // next_state.set(GameState::FightArena);
    }
}
