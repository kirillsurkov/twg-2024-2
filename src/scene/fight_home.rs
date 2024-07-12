use std::error::Error;

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

use crate::{
    battle_bridge::{BattleResource, RoundCaptureResource},
    component::{game_timer::GameTimer, home::Home},
    hero::HeroesRoot,
    scene::UiRoot,
    ui::fight_home_layout::FightHomeLayout,
};

use super::{landing::HeroSelected, GameState, LocalSchedule, Root};

#[derive(Resource)]
struct State {}

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
    mut game_timer: ResMut<GameTimer>,
    selected: Res<HeroSelected>,
    root: Query<Entity, Added<Root>>,
) -> Result<(), Box<dyn Error>> {
    let root = root.get_single()?;
    println!("FIGHT HOME INIT FOR {}", selected.id);
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

    commands.spawn((UiRoot, FightHomeLayout));

    commands.insert_resource(State {});

    game_timer.restart(5.0, false);

    Ok(())
}

fn update(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut battle: ResMut<BattleResource>,
    game_timer: Res<GameTimer>,
) {
    if game_timer.fired {
        commands.insert_resource(RoundCaptureResource(battle.round()));
        next_state.set(GameState::FightArena);
    }
}
