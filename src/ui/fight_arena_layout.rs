use bevy::prelude::*;

use crate::{hero::HeroId, scene::landing::HeroSelected};

use super::{
    avatar::AvatarRoot,
    fight_home_layout::RoundsCount,
    game_timer::GameTimerRoot,
    hp_mana_bars::HpManaBarsRoot,
    players::PlayersRoot,
    screen::{
        ScreenBodyBot, ScreenBodyRoot, ScreenBodyTop, ScreenBottom, ScreenFooter, ScreenHeader,
        ScreenMain, ScreenRoot,
    },
    stats::StatsRoot,
    LocalSchedule,
};

#[derive(Component)]
pub struct FightArenaLayout;

impl Plugin for FightArenaLayout {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, init.run_if(resource_exists::<HeroSelected>));
    }
}

fn init(
    mut commands: Commands,
    selected: Res<HeroSelected>,
    query: Query<Entity, Added<FightArenaLayout>>,
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
                                        p.spawn((NodeBundle::default(), HpManaBarsRoot));
                                    });
                            });
                        p.spawn((NodeBundle::default(), ScreenFooter));
                        p.spawn((NodeBundle::default(), ScreenBottom))
                            .with_children(|p| {
                                p.spawn((NodeBundle::default(), AvatarRoot::Left));
                                p.spawn((NodeBundle::default(), AvatarRoot::Right));
                            });
                    });
            });
    }
}
