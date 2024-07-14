use std::cmp::Reverse;

use bevy::prelude::*;

use crate::{
    battle_bridge::{BattleResource, HeroesResource, RoundCaptureResource},
    hero::HeroId,
    scene::{
        avatars::AvatarsResource,
        landing::{HeroSelected, HeroWatch},
    },
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
                init_player_avatar,
                init_player_body,
                init_player_name,
                init_player_stats,
                init_player_money,
                update_player_money,
                init_player_attack,
                update_player_attack,
                init_player_hp,
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
                    margin: UiRect::bottom(Val::Auto),
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
    round: Option<Res<RoundCaptureResource>>,
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
                let players = if let Some(round) = round.as_ref() {
                    round
                        .0
                        .iter()
                        .flat_map(|r| vec![r.player1, r.player2])
                        .map(|p| {
                            battle
                                .players
                                .iter()
                                .find(|player| player.hero.id == p)
                                .unwrap()
                        })
                        .collect()
                } else {
                    let mut players = battle.players.iter().collect::<Vec<_>>();
                    players.sort_by_key(|p| Reverse((p.hp, p.money)));
                    players
                };

                let last = players.len() - 1;
                for (i, player) in players.iter().enumerate() {
                    p.spawn((
                        NodeBundle::default(),
                        HeroId(player.hero.id.to_string()),
                        PlayerRoot(round.is_some() && i != last && i % 2 == 1),
                    ));
                }
            });
    }
}

#[derive(Component)]
struct PlayerRoot(bool);

fn init_player_root(
    mut commands: Commands,
    query: Query<(Entity, &HeroId, &PlayerRoot), Added<PlayerRoot>>,
) {
    for (entity, id, root) in query.iter() {
        commands
            .entity(entity)
            .insert(ButtonBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    width: Val::Px(HEIGHT * 4.0 + 10.0),
                    height: Val::Px(HEIGHT),
                    margin: UiRect::new(
                        Val::Px(0.0),
                        Val::Px(0.0),
                        Val::Px(5.0),
                        Val::Px(10.0 * root.0 as u32 as f32),
                    ),
                    column_gap: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((NodeBundle::default(), HeroId(id.to_string()), PlayerAvatar));
                p.spawn((NodeBundle::default(), HeroId(id.to_string()), PlayerBody));
                p.spawn((NodeBundle::default(), HeroId(id.to_string()), PlayerHp));
            });
    }
}

#[derive(Component)]
struct PlayerListSelected(String);

fn update_player_root(
    mut commands: Commands,
    mut query: Query<(Entity, &HeroId, &Interaction, &mut BackgroundColor), With<PlayerRoot>>,
    selected: Res<HeroSelected>,
) {
    for (entity, id, act, mut color) in query.iter_mut() {
        let base = if selected.id == id.0 {
            Color::MIDNIGHT_BLUE
        } else {
            Color::BLACK
        }
        .with_a(0.5);
        match act {
            Interaction::Hovered => {
                commands
                    .entity(entity)
                    .insert(PlayerListSelected(id.to_string()));
                let tint = 0.25;
                *color = Color::rgba(
                    base.r() + (tint * (1.0 - base.r())),
                    base.g() + (tint * (1.0 - base.g())),
                    base.b() + (tint * (1.0 - base.b())),
                    base.a(),
                )
                .into();
            }
            Interaction::Pressed => {
                commands.insert_resource(HeroWatch { id: id.to_string() });
                let tint = 0.30;
                *color = Color::rgba(
                    base.r() + (tint * (1.0 - base.r())),
                    base.g() + (tint * (1.0 - base.g())),
                    base.b() + (tint * (1.0 - base.b())),
                    base.a(),
                )
                .into();
            }
            Interaction::None => {
                commands.entity(entity).remove::<PlayerListSelected>();
                *color = base.into();
            }
        }
    }
}

#[derive(Component)]
struct PlayerAvatar;

fn init_player_avatar(
    mut commands: Commands,
    avatars: Res<AvatarsResource>,
    query: Query<(Entity, &HeroId), With<PlayerAvatar>>,
    images: Query<(), (With<PlayerAvatar>, With<UiImage>)>,
) {
    if !avatars.is_changed() && !images.is_empty() {
        return;
    }

    for (entity, id) in query.iter() {
        commands.entity(entity).insert(ImageBundle {
            image: UiImage {
                texture: avatars.thumbnails.get(&id.0).unwrap().clone(),
                ..Default::default()
            },
            style: Style {
                display: Display::Flex,
                width: Val::Px(HEIGHT),
                height: Val::Px(HEIGHT),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

#[derive(Component)]
struct PlayerBody;

fn init_player_body(mut commands: Commands, query: Query<(Entity, &HeroId), Added<PlayerBody>>) {
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
    heroes: Res<HeroesResource>,
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
                    heroes.iter().find(|(h, _)| h.id == id.0).unwrap().0.name,
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
    query: Query<Entity, Added<PlayerMoney>>,
) {
    for entity in query.iter() {
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
    query: Query<Entity, Added<PlayerAttack>>,
) {
    for entity in query.iter() {
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
struct PlayerHp;

fn init_player_hp(
    mut commands: Commands,
    assets: Res<UiAssets>,
    query: Query<Entity, Added<PlayerHp>>,
) {
    for entity in query.iter() {
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
            }),
        );
    }
}

fn update_player_footer(
    mut query: Query<(&HeroId, &mut Text), With<PlayerHp>>,
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
