use bevy::prelude::*;

use crate::{battle::fight, battle_bridge::RoundCaptureResource, scene::landing::HeroSelected};

use super::{game_timer::GameTimer, LocalSchedule};

#[derive(Resource, Deref)]
pub struct FightState(pub fight::State);

pub struct FightStatePlugin;

impl Plugin for FightStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            update.run_if(resource_exists::<RoundCaptureResource>),
        );
    }
}

fn update(
    mut commands: Commands,
    capture: Res<RoundCaptureResource>,
    selected: Res<HeroSelected>,
    game_timer: Res<GameTimer>,
    time: Res<Time>,
) {
    if game_timer.red {
        return;
    }

    let capture = capture.by_player(&selected.id).unwrap();
    let fight = &capture.fight_capture;
    if let Some(fight_state) =
        fight.state(game_timer.value, game_timer.value + time.delta_seconds())
    {
        commands.insert_resource(FightState(fight_state));
    }
}
