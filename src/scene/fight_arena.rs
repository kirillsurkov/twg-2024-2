use std::{error::Error, f32::consts::PI};

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

use crate::{
    battle::fight::DURATION,
    battle_bridge::{BattleResource, RoundCaptureResource},
    component::{arena::Arena, game_timer::GameTimer},
    hero::HeroesRoot,
    scene::UiRoot,
    ui::fight_arena_layout::FightArenaLayout,
};

use super::{
    landing::{HeroSelected, HeroWatch},
    GameState, LocalSchedule, Root,
};

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
    root: Query<Entity, Added<Root>>,
) -> Result<(), Box<dyn Error>> {
    let root = root.get_single()?;

    commands.entity(root).with_children(|p| {
        p.spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 7.5, 8.5))
                    .looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
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
                ..Default::default()
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
    mut battle: ResMut<BattleResource>,
    selected: Res<HeroSelected>,
) {
    if game_timer.fired {
        if game_timer.red {
            game_timer.restart(99999.0, false);

            commands.insert_resource(HeroWatch {
                id: selected.id.clone(),
            });
            commands.remove_resource::<RoundCaptureResource>();
            battle.apply();
            for player in battle
                .players
                .iter()
                .map(|p| p.hero.id)
                .filter(|id| *id != selected.id)
                .collect::<Vec<_>>()
            {
                battle.ai(player);
            }
            next_state.set(GameState::FightHome);
        } else {
            game_timer.restart(3.0, true);
        }
    }
}
