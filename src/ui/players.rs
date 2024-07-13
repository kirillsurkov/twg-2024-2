use bevy::prelude::*;

use crate::{
    battle_bridge::BattleResource,
    hero::HeroId,
    scene::landing::{HeroSelected, HeroWatch},
};

use super::{LocalSchedule, UiAssets, DCOLOR};

pub struct PlayersPlugin;

const HEIGHT: f32 = 50.0;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (
                init_players_root,
                init_players_list,
                init_player_root,
                update_player_root,
                init_player_header,
                init_player_body,
                init_player_name,
                init_player_stats,
                init_player_money,
                update_player_money,
                init_player_attack,
                update_player_attack,
                init_player_footer,
                update_player_footer,
                init_player_info_root,
                update_player_info_root,
                init_player_info,
                update_player_info,
            )
                .run_if(resource_exists::<BattleResource>),
        );
    }
}

#[derive(Component)]
pub struct PlayersRoot;

fn init_players_root(mut commands: Commands, query: Query<Entity, Added<PlayersRoot>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    margin: UiRect::right(Val::Auto),
                    ..Default::default()
                },
                // background_color: DCOLOR,
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((NodeBundle::default(), PlayersList));
                p.spawn((NodeBundle::default(), PlayerInfoRoot));
            });
    }
}

#[derive(Component)]
struct PlayersList;

fn init_players_list(
    mut commands: Commands,
    query: Query<Entity, Added<PlayersList>>,
    battle: Res<BattleResource>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::right(Val::Auto),
                    ..Default::default()
                },
                // background_color: DCOLOR,
                ..Default::default()
            })
            .with_children(|p| {
                battle.players.iter().for_each(|player| {
                    p.spawn((
                        NodeBundle::default(),
                        HeroId(player.hero.id.to_string()),
                        PlayerRoot,
                    ));
                })
            });
    }
}

#[derive(Component)]
struct PlayerRoot;

fn init_player_root(mut commands: Commands, query: Query<(Entity, &HeroId), Added<PlayerRoot>>) {
    for (entity, id) in query.iter() {
        commands
            .entity(entity)
            .insert(ButtonBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    width: Val::Px(HEIGHT * 4.0),
                    height: Val::Px(HEIGHT),
                    margin: UiRect::top(Val::Px(5.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((NodeBundle::default(), HeroId(id.to_string()), PlayerHeader));
                p.spawn((NodeBundle::default(), HeroId(id.to_string()), PlayerBody));
                p.spawn((NodeBundle::default(), HeroId(id.to_string()), PlayerFooter));
            });
    }
}

#[derive(Component)]
struct PlayerListSelected(String);

fn update_player_root(
    mut commands: Commands,
    mut query: Query<(Entity, &HeroId, &Interaction, &mut BackgroundColor), With<PlayerRoot>>,
) {
    for (entity, id, act, mut color) in query.iter_mut() {
        match act {
            Interaction::Hovered => {
                commands
                    .entity(entity)
                    .insert(PlayerListSelected(id.to_string()));
                *color = Color::WHITE.with_a(0.05).into();
            }
            Interaction::Pressed => {
                commands.insert_resource(HeroWatch { id: id.to_string() });
                *color = Color::WHITE.with_a(0.07).into();
            }
            Interaction::None => {
                commands.entity(entity).remove::<PlayerListSelected>();
                *color = Color::NONE.into();
            }
        }
    }
}

#[derive(Component)]
struct PlayerHeader;

fn init_player_header(
    mut commands: Commands,
    battle: Res<BattleResource>,
    query: Query<(Entity, &HeroId), Added<PlayerHeader>>,
) {
    for (entity, id) in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Px(HEIGHT),
                height: Val::Px(HEIGHT),
                ..Default::default()
            },
            background_color: DCOLOR,
            ..Default::default()
        });
    }
}

#[derive(Component)]
struct PlayerBody;

fn init_player_body(
    mut commands: Commands,
    selected: Res<HeroSelected>,
    query: Query<(Entity, &HeroId), Added<PlayerBody>>,
) {
    for (entity, id) in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Px(HEIGHT * 2.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                background_color: if selected.id == id.0 {
                    Color::GREEN.with_a(0.1).into()
                } else {
                    DCOLOR
                },
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((NodeBundle::default(), HeroId(id.to_string()), PlayerName));
                p.spawn((NodeBundle::default(), HeroId(id.to_string()), PlayerStats));
            });
    }
}

#[derive(Component)]
struct PlayerName;

fn init_player_name(
    mut commands: Commands,
    assets: Res<UiAssets>,
    query: Query<(Entity, &HeroId), Added<PlayerName>>,
) {
    for (entity, id) in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    &id.0,
                    TextStyle {
                        font: assets.font_comic.clone_weak(),
                        font_size: 25.0,
                        ..Default::default()
                    },
                ));
            });
    }
}

#[derive(Component)]
struct PlayerStats;

fn init_player_stats(mut commands: Commands, query: Query<(Entity, &HeroId), Added<PlayerStats>>) {
    for (entity, id) in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        width: Val::Percent(70.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::FlexStart,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|p| {
                    p.spawn((NodeBundle::default(), HeroId(id.to_string()), PlayerMoney));
                });

                p.spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_grow: 1.0,
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::FlexStart,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|p| {
                    p.spawn((NodeBundle::default(), HeroId(id.to_string()), PlayerAttack));
                });
            });
    }
}

#[derive(Component)]
struct PlayerMoney;

fn init_player_money(
    mut commands: Commands,
    assets: Res<UiAssets>,
    query: Query<(Entity, &HeroId), Added<PlayerMoney>>,
) {
    for (entity, id) in query.iter() {
        commands.entity(entity).insert(TextBundle::from_section(
            "",
            TextStyle {
                font: assets.font_comic.clone_weak(),
                font_size: 25.0,
                color: Color::GREEN,
            },
        ));
    }
}

fn update_player_money(
    mut query: Query<(&HeroId, &mut Text), With<PlayerMoney>>,
    battle: Res<BattleResource>,
) {
    for (id, mut text) in query.iter_mut() {
        let player = battle.players.iter().find(|p| p.hero.id == id.0).unwrap();
        text.sections[0].value = format!("{}$", player.money);
    }
}

#[derive(Component)]
struct PlayerAttack;

fn init_player_attack(
    mut commands: Commands,
    assets: Res<UiAssets>,
    query: Query<(Entity, &HeroId), Added<PlayerAttack>>,
) {
    for (entity, id) in query.iter() {
        commands.entity(entity).insert(TextBundle::from_section(
            "",
            TextStyle {
                font: assets.font_comic.clone_weak(),
                font_size: 25.0,
                color: Color::RED,
            },
        ));
    }
}

fn update_player_attack(
    mut query: Query<(&HeroId, &mut Text), With<PlayerAttack>>,
    battle: Res<BattleResource>,
) {
    for (id, mut text) in query.iter_mut() {
        let player = battle.players.iter().find(|p| p.hero.id == id.0).unwrap();
        text.sections[0].value = format!("{}", player.attack);
    }
}

#[derive(Component)]
struct PlayerFooter;

fn init_player_footer(
    mut commands: Commands,
    assets: Res<UiAssets>,
    query: Query<(Entity, &HeroId), Added<PlayerFooter>>,
) {
    for (entity, id) in query.iter() {
        commands.entity(entity).insert(
            TextBundle::from_section(
                "50",
                TextStyle {
                    font: assets.font_comic.clone_weak(),
                    font_size: 50.0,
                    ..Default::default()
                },
            )
            .with_style(Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                flex_grow: 1.0,
                height: Val::Percent(100.0),
                ..Default::default()
            })
            .with_background_color(DCOLOR.0),
        );
    }
}

fn update_player_footer(
    mut query: Query<(&HeroId, &mut Text), With<PlayerFooter>>,
    battle: Res<BattleResource>,
) {
    for (id, mut text) in query.iter_mut() {
        let player = battle.players.iter().find(|p| p.hero.id == id.0).unwrap();
        text.sections[0].value = format!("{}", player.hp);
    }
}

#[derive(Component)]
struct PlayerInfoRoot;

fn init_player_info_root(mut commands: Commands, query: Query<Entity, Added<PlayerInfoRoot>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    width: Val::Px(HEIGHT * 4.0),
                    flex_grow: 1.0,
                    margin: UiRect::new(Val::Px(5.0), Val::Px(0.0), Val::Px(5.0), Val::Px(0.0)),
                    ..Default::default()
                },
                visibility: Visibility::Hidden,
                background_color: DCOLOR,
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((NodeBundle::default(), PlayerInfo));
            });
    }
}

fn update_player_info_root(
    mut query: Query<&mut Visibility, With<PlayerInfoRoot>>,
    selected: Query<&PlayerListSelected>,
) {
    let selected = selected.get_single();
    for mut visibility in query.iter_mut() {
        *visibility = match selected {
            Ok(_) => Visibility::Inherited,
            Err(_) => Visibility::Hidden,
        }
    }
}

#[derive(Component)]
struct PlayerInfo;

fn init_player_info(
    mut commands: Commands,
    assets: Res<UiAssets>,
    query: Query<Entity, Added<PlayerInfo>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(TextBundle::from_section(
            "",
            TextStyle {
                font: assets.font_comic.clone_weak(),
                font_size: 50.0,
                ..Default::default()
            },
        ));
    }
}

fn update_player_info(
    mut query: Query<&mut Text, With<PlayerInfo>>,
    selected: Query<&PlayerListSelected>,
) {
    let Ok(selected) = selected.get_single() else {
        return;
    };

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("id: {}", selected.0);
    }
}
