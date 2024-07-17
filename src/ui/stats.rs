use bevy::prelude::*;

use crate::{
    battle::card::CardBranch,
    battle_bridge::{branch_to_color, BattleResource},
    hero::HeroId,
};

use super::LocalSchedule;

pub struct StatsPlugin;

pub const WIDTH: f32 = 300.0;
const ROW_HEIGHT: f32 = 30.0;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (
                init_stats_root,
                init_stat_root,
                update_stat_count.after(init_stat_root),
            )
                .run_if(resource_exists::<BattleResource>),
        );
    }
}

#[derive(Component)]
pub struct StatsRoot;

fn init_stats_root(mut commands: Commands, query: Query<Entity, Added<StatsRoot>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::new(Val::Auto, Val::ZERO, Val::ZERO, Val::Auto),
                    width: Val::Px(WIDTH),
                    row_gap: Val::Px(10.0),
                    padding: UiRect::vertical(Val::Px(10.0)),
                    ..Default::default()
                },
                background_color: Color::BLACK.with_a(0.5).into(),
                ..Default::default()
            })
            .despawn_descendants()
            .with_children(|p| {
                let separator = NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(3.0),
                        margin: UiRect::horizontal(Val::Auto),
                        ..Default::default()
                    },
                    background_color: Color::BLACK.with_a(0.3).into(),
                    ..Default::default()
                };
                p.spawn((NodeBundle::default(), StatHolder(CardBranch::Attack)));
                p.spawn(separator.clone());
                p.spawn((NodeBundle::default(), StatHolder(CardBranch::Regen)));
                p.spawn(separator.clone());
                p.spawn((NodeBundle::default(), StatHolder(CardBranch::Hp)));
                p.spawn(separator.clone());
                p.spawn((NodeBundle::default(), StatHolder(CardBranch::Mana)));
                p.spawn(separator.clone());
                p.spawn((NodeBundle::default(), StatHolder(CardBranch::Crit)));
                p.spawn(separator.clone());
                p.spawn((NodeBundle::default(), StatHolder(CardBranch::Evasion)));
            });
    }
}

#[derive(Component)]
pub struct StatHolder(CardBranch);

fn init_stat_root(
    mut commands: Commands,
    query: Query<(Entity, &StatHolder, &Parent), Added<StatHolder>>,
) {
    for (entity, stat, parent) in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    height: Val::Px(ROW_HEIGHT),
                    align_items: AlignItems::Center,
                    padding: UiRect::horizontal(Val::Px(10.0)),
                    column_gap: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        width: Val::Px(ROW_HEIGHT),
                        height: Val::Px(ROW_HEIGHT),
                        border: UiRect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    border_color: branch_to_color(&stat.0).with_a(0.5).into(),
                    background_color: branch_to_color(&stat.0).with_a(0.2).into(),
                    ..Default::default()
                });
                p.spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(60.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    // background_color: Color::RED.with_a(0.1).into(),
                    ..Default::default()
                })
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        match stat.0 {
                            CardBranch::Attack => "Attack",
                            CardBranch::Regen => "Regen",
                            CardBranch::Hp => "HP",
                            CardBranch::Mana => "Mana",
                            CardBranch::Crit => "Crit",
                            CardBranch::Evasion => "Evasion",
                        },
                        TextStyle {
                            color: branch_to_color(&stat.0),
                            font_size: 20.0,
                            ..Default::default()
                        },
                    ));
                });
                p.spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        flex_grow: 1.0,
                        column_gap: Val::Px(10.0),
                        ..Default::default()
                    },
                    // background_color: DCOLOR,
                    ..Default::default()
                })
                .with_children(|p| {
                    p.spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            width: Val::Percent(50.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::FlexEnd,
                            ..Default::default()
                        },
                        // background_color: DCOLOR,
                        ..Default::default()
                    })
                    .with_children(|p| {
                        p.spawn((
                            TextBundle::from_section(
                                "",
                                TextStyle {
                                    font_size: 20.0,
                                    ..Default::default()
                                },
                            ),
                            StatCount(parent.get(), stat.0.clone(), BranchInfo::Current),
                        ));
                    });
                    p.spawn(TextBundle::from_section(
                        "/",
                        TextStyle {
                            font_size: 20.0,
                            ..Default::default()
                        },
                    ));
                    p.spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            width: Val::Percent(50.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::FlexStart,
                            ..Default::default()
                        },
                        // background_color: DCOLOR,
                        ..Default::default()
                    })
                    .with_children(|p| {
                        p.spawn((
                            TextBundle::from_section(
                                "",
                                TextStyle {
                                    font_size: 20.0,
                                    ..Default::default()
                                },
                            ),
                            StatCount(parent.get(), stat.0.clone(), BranchInfo::Max),
                        ));
                    });
                });
            });
    }
}

enum BranchInfo {
    Current,
    Max,
}

#[derive(Component)]
pub struct StatCount(Entity, CardBranch, BranchInfo);

fn update_stat_count(
    mut query: Query<(&StatCount, &mut Text)>,
    battle: Res<BattleResource>,
    hero_ids: Query<&HeroId>,
) {
    for (StatCount(root, branch, info), mut text) in query.iter_mut() {
        let Ok(hero_id) = hero_ids.get(*root) else {
            continue;
        };

        let player = battle
            .players
            .iter()
            .find(|player| player.hero.id == hero_id.0)
            .unwrap();

        let value = match info {
            BranchInfo::Current => player.branch_value(branch),
            BranchInfo::Max => battle.branch_max(branch),
        };
        text.sections[0].value = format!("{}", value);
    }
}
