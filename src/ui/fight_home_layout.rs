use bevy::prelude::*;

use super::{
    cards::CardsRoot,
    game_timer::GameTimerRoot,
    players::PlayersRoot,
    screen::{ScreenBody, ScreenFooter, ScreenHeader, ScreenMain, ScreenRoot, ScreenSide},
    LocalSchedule,
};

#[derive(Component)]
pub struct FightHomeLayout;

impl Plugin for FightHomeLayout {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, init);
    }
}

fn init(mut commands: Commands, query: Query<Entity, Added<FightHomeLayout>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert((NodeBundle::default(), ScreenRoot))
            .with_children(|p| {
                p.spawn((NodeBundle::default(), ScreenMain))
                    .with_children(|p| {
                        p.spawn((NodeBundle::default(), ScreenHeader))
                            .with_children(|p| {
                                p.spawn((NodeBundle::default(), GameTimerRoot));
                            });
                        p.spawn((NodeBundle::default(), ScreenBody))
                            .with_children(|p| {
                                p.spawn((NodeBundle::default(), PlayersRoot));
                                p.spawn((NodeBundle::default(), CardsRoot));
                            });
                        p.spawn((NodeBundle::default(), ScreenFooter));
                    });
                p.spawn((NodeBundle::default(), ScreenSide));
            });
    }
}
