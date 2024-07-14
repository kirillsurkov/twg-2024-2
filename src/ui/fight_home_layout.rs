use bevy::prelude::*;

use super::{
    avatar::AvatarRoot,
    cards::CardsRoot,
    game_timer::GameTimerRoot,
    players::PlayersRoot,
    screen::{
        ScreenBodyBot, ScreenBodyRoot, ScreenBodyTop, ScreenBottom, ScreenFooter, ScreenHeader,
        ScreenMain, ScreenRoot,
    },
    stats::StatsRoot,
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
                        p.spawn((NodeBundle::default(), ScreenBodyRoot))
                            .with_children(|p| {
                                p.spawn((NodeBundle::default(), ScreenBodyTop))
                                    .with_children(|p| {
                                        p.spawn((NodeBundle::default(), PlayersRoot));
                                        p.spawn((NodeBundle::default(), StatsRoot));
                                    });
                                p.spawn((NodeBundle::default(), ScreenBodyBot))
                                    .with_children(|p| {
                                        p.spawn((NodeBundle::default(), CardsRoot));
                                    });
                            });
                        p.spawn((NodeBundle::default(), ScreenFooter));
                        p.spawn((NodeBundle::default(), ScreenBottom))
                            .with_children(|p| {
                                p.spawn((NodeBundle::default(), AvatarRoot::Right));
                            });
                    });
            });
    }
}
