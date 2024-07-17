use bevy::prelude::*;

use crate::{battle_bridge::BattleResource, hero::HeroId, scene::landing::HeroSelected};

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
        app.add_systems(LocalSchedule, init.run_if(resource_exists::<HeroSelected>));
        app.add_systems(
            LocalSchedule,
            update_rounds.run_if(resource_exists::<BattleResource>),
        );
    }
}

#[derive(Component)]
pub struct RoundsCount;

fn update_rounds(mut query: Query<&mut Text, With<RoundsCount>>, battle: Res<BattleResource>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Round {}", battle.round);
    }
}

fn init(
    mut commands: Commands,
    selected: Res<HeroSelected>,
    query: Query<Entity, Added<FightHomeLayout>>,
) {
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
                                p.spawn(NodeBundle {
                                    style: Style {
                                        display: Display::Flex,
                                        height: Val::Percent(100.0),
                                        margin: UiRect::right(Val::Auto),
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    background_color: Color::BLACK.with_a(0.5).into(),
                                    ..Default::default()
                                })
                                .with_children(|p| {
                                    p.spawn((
                                        TextBundle::from_section(
                                            "",
                                            TextStyle {
                                                font_size: 50.0,
                                                color: Color::YELLOW,
                                                ..Default::default()
                                            },
                                        ),
                                        RoundsCount,
                                    ));
                                });
                            });
                        p.spawn((NodeBundle::default(), ScreenBodyRoot))
                            .with_children(|p| {
                                p.spawn((NodeBundle::default(), ScreenBodyTop))
                                    .with_children(|p| {
                                        p.spawn((NodeBundle::default(), PlayersRoot));
                                        p.spawn((
                                            NodeBundle::default(),
                                            StatsRoot,
                                            HeroId(selected.id.clone()),
                                        ));
                                    });
                                p.spawn((NodeBundle::default(), ScreenBodyBot))
                                    .with_children(|p| {
                                        p.spawn((NodeBundle::default(), CardsRoot));
                                    });
                            });
                        p.spawn((NodeBundle::default(), ScreenFooter));
                        p.spawn((NodeBundle::default(), ScreenBottom))
                            .with_children(|p| {
                                p.spawn((NodeBundle::default(), AvatarRoot::Left));
                            });
                    });
            });
    }
}
