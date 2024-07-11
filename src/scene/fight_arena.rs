use std::{error::Error, f32::consts::PI};

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

use crate::{
    battle::fight::{self, DURATION},
    battle_bridge::RoundCaptureResource,
    component::arena::Arena,
    hero::HeroesRoot,
    scene::UiRoot,
    ui::fight_arena_layout::FightArenaLayout,
};

use super::{landing::HeroSelected, GameState, LocalSchedule, Root};

#[derive(Resource)]
pub struct State {
    pub timer: f32,
    pub timer_max: f32,
    pub current_state: Option<fight::State>,
}

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
    selected: Res<HeroSelected>,
    root: Query<Entity, Added<Root>>,
) -> Result<(), Box<dyn Error>> {
    let root = root.get_single()?;
    println!("FIGHT ARENA INIT FOR {}", selected.id);

    commands.insert_resource(State {
        timer: 0.0,
        timer_max: DURATION,
        current_state: None,
    });
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

    Ok(())
}

fn update(
    mut next_state: ResMut<NextState<GameState>>,
    mut state: ResMut<State>,
    selected: Res<HeroSelected>,
    capture: Res<RoundCaptureResource>,
    time: Res<Time>,
) {
    let capture = capture.by_player(&selected.id).unwrap();
    let fight = &capture.fight_capture;
    if let Some(fight_state) = fight.state(state.timer, state.timer + time.delta_seconds()) {
        println!("{fight_state:#?}");
        state.current_state = Some(fight_state);
    }
    state.timer += time.delta_seconds();
    if state.timer >= DURATION + 3.0 {
        next_state.set(GameState::FightHome);
    }
}
