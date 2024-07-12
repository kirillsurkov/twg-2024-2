use std::{error::Error, f32::consts::PI};

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

use crate::{
    battle::fight::DURATION,
    battle_bridge::RoundCaptureResource,
    component::{arena::Arena, game_timer::GameTimer},
    hero::HeroesRoot,
    scene::UiRoot,
    ui::{fight_arena_layout::FightArenaLayout, FightState},
};

use super::{landing::HeroSelected, GameState, LocalSchedule, Root};

#[derive(Resource)]
struct State {}

pub struct FightArena;

impl Plugin for FightArena {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (init.map(drop), update.run_if(resource_exists::<State>))
                .run_if(in_state(GameState::FightArena)),
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
    println!("FIGHT ARENA INIT FOR {}", selected.id);

    commands.entity(root).with_children(|p| {
        p.spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 7.5, 7.5))
                    .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: PI * 5.0 / 12.0,
                    ..Default::default()
                }),
                ..Default::default()
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
                .looking_at(Vec3::new(0.05, -0.15, -0.25), Vec3::Y),
            ..Default::default()
        });

        p.spawn((Arena {}, HeroesRoot));
    });

    commands.spawn((UiRoot, FightArenaLayout));

    commands.insert_resource(State {});

    game_timer.restart(DURATION, false);

    Ok(())
}

fn update(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_timer: ResMut<GameTimer>,
    selected: Res<HeroSelected>,
    capture: Res<RoundCaptureResource>,
    time: Res<Time>,
) {
    if !game_timer.red {
        let capture = capture.by_player(&selected.id).unwrap();
        let fight = &capture.fight_capture;
        if let Some(fight_state) =
            fight.state(game_timer.value, game_timer.value + time.delta_seconds())
        {
            commands.insert_resource(FightState {
                current: fight_state,
            });
        }
    }

    if game_timer.fired && game_timer.red {
        next_state.set(GameState::FightHome);
    }

    if game_timer.fired {
        game_timer.restart(3.0, true);
    }
}
