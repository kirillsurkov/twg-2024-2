use bevy::prelude::*;

use crate::{
    battle::fight::{self, Owner},
    battle_bridge::{BattleResource, RoundCaptureResource},
    scene::landing::HeroWatch,
};

use super::{game_timer::GameTimer, LocalSchedule};

#[derive(Resource, Deref)]
pub struct FightState(pub fight::State);

pub struct FightStatePlugin;

impl Plugin for FightStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (update1, update2).run_if(resource_exists::<RoundCaptureResource>),
        );
    }
}

fn update1(
    mut battle: ResMut<BattleResource>,
    capture: Res<RoundCaptureResource>,
    game_timer: Res<GameTimer>,
    time: Res<Time>,
) {
    if game_timer.red {
        return;
    }

    for capture in &capture.0 {
        let Some(fight_state) = capture
            .fight_capture
            .state(game_timer.value, game_timer.value + time.delta_seconds())
        else {
            continue;
        };

        let Some(winner) = fight_state.winner else {
            continue;
        };

        let (winner, loser) = match winner {
            Owner::Fighter1 => (capture.player1, capture.player2),
            Owner::Fighter2 => (capture.player2, capture.player1),
        };

        let attack = battle
            .players
            .iter()
            .find(|p| p.hero.id == winner)
            .unwrap()
            .attack;

        battle
            .players
            .iter_mut()
            .find(|p| p.hero.id == winner)
            .map(|p| {
                p.attack = (p.attack + 1).min(10);
                p.money += (p.money / 100).min(10) * 10;
                p.money += 250;
                p.money += 50;
            });

        battle
            .players
            .iter_mut()
            .find(|p| p.hero.id == loser)
            .map(|p| {
                p.attack = 3;
                p.hp -= attack;
                p.money += (p.money / 100).min(10) * 10;
                p.money += 250;
                p.money += attack * 15;
            });
    }
}

fn update2(
    mut commands: Commands,
    capture: Res<RoundCaptureResource>,
    watch: Res<HeroWatch>,
    game_timer: Res<GameTimer>,
    time: Res<Time>,
) {
    let capture = capture.by_player(&watch.id).unwrap();
    let fight = &capture.fight_capture;
    if game_timer.red || game_timer.value >= fight.duration() {
        commands.insert_resource(FightState(fight.last()));
    } else if let Some(fight_state) =
        fight.state(game_timer.value, game_timer.value + time.delta_seconds())
    {
        commands.insert_resource(FightState(fight_state));
    }
}
