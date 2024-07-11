use std::error::Error;

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

use crate::{
    battle::fight::DURATION, battle_bridge::RoundCaptureResource, component::arena::Arena,
    hero::HeroesRoot,
};

use super::{landing::HeroSelected, GameState, LocalSchedule, Root};

#[derive(Resource)]
struct State {
    timer: f32,
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
    commands.insert_resource(State { timer: 0.0 });
    commands.entity(root).with_children(|p| {
        p.spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(-2.0, 10.0, 10.0))
                    .looking_at(Vec3::new(0.0, 0.0, -5.0), Vec3::Y),
                ..default()
            },
            BloomSettings::default(),
        ));

        p.spawn((Arena {}, HeroesRoot));
    });
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
        println!("{} vs {}", capture.player1, capture.player2);
        println!("{fight_state:#?}");
    }
    state.timer += time.delta_seconds();
    if state.timer >= DURATION + 3.0 {
        next_state.set(GameState::FightHome);
    }
}
