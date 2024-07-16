use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::{battle::card::CardOps, battle_bridge::BattleResource, scene::landing::HeroSelected};

use super::{stats, LocalSchedule, DCOLOR};

const HEIGHT: f32 = 300.0;
const CONTROLS_WIDTH: f32 = 200.0;
const CARD_WIDTH: f32 = 200.0;
const NAME_HEIGHT: f32 = 30.0;
const FOOTER_HEIGHT: f32 = 30.0;

pub struct CardsPlugin;

impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (
                init_root,
                init_cards_holder,
                update_cards_holder,
                init_card_holder,
                update_card_holder,
                (
                    init_card_header,
                    init_card_levels,
                    init_card_level_active,
                    init_card_level_inactive,
                    init_card_level_blink,
                    update_card_level_blink,
                    init_card_name,
                    init_card_desc,
                    init_card_footer,
                )
                    .before(update_card_holder),
                init_cards_controls,
                init_cards_control,
            )
                .run_if(resource_exists::<BattleResource>),
        );
    }
}

#[derive(Component)]
pub struct CardsRoot;

fn init_root(mut commands: Commands, query: Query<Entity, Added<CardsRoot>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    margin: UiRect::right(Val::Px(stats::WIDTH)),
                    column_gap: Val::Px(5.0),
                    ..Default::default()
                },
                background_color: DCOLOR,
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((NodeBundle::default(), CardsHolder(vec![])));
                p.spawn((NodeBundle::default(), CardsControls));
            });
    }
}

#[derive(Component)]
struct CardsHolder(Vec<(bool, Box<dyn CardOps>)>);

fn init_cards_holder(mut commands: Commands, query: Query<Entity, Added<CardsHolder>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                column_gap: Val::Px(5.0),
                ..Default::default()
            },
            // background_color: DCOLOR,
            ..Default::default()
        });
    }
}

fn update_cards_holder(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CardsHolder)>,
    battle: Res<BattleResource>,
    selected: Res<HeroSelected>,
) {
    for (entity, mut holder) in query.iter_mut() {
        let player = battle
            .players
            .iter()
            .find(|player| player.hero.id == selected.id)
            .unwrap();
        if !holder
            .0
            .iter()
            .map(|(_, c)| c)
            .eq(player.cards_reserved.iter().map(|(_, c)| c))
        {
            println!("CARDS");
            holder.0 = player.cards_reserved.clone();
            commands
                .entity(entity)
                .despawn_descendants()
                .with_children(|p| {
                    for (i, (_, card)) in holder.0.iter().enumerate() {
                        p.spawn((NodeBundle::default(), CardHolder(i)))
                            .with_children(|p| {
                                p.spawn((NodeBundle::default(), CardHeader));
                                p.spawn((
                                    NodeBundle::default(),
                                    CardLevels(i, card.level(), card.max_level()),
                                ));
                                p.spawn((NodeBundle::default(), CardName(card.name())));
                                p.spawn((NodeBundle::default(), CardDesc(card.desc())));
                                p.spawn((NodeBundle::default(), CardFooter(card.cost())));
                            });
                    }
                });
        }
    }
}

#[derive(Component)]
struct CardHolder(usize);

fn init_card_holder(mut commands: Commands, query: Query<Entity, Added<CardHolder>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(ButtonBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Px(CARD_WIDTH),
                height: Val::Px(HEIGHT),
                row_gap: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

fn update_card_holder(
    mut battle: ResMut<BattleResource>,
    mut query: Query<(&mut BackgroundColor, &CardHolder, &Interaction)>,
    selected: Res<HeroSelected>,
) {
    for (mut color, CardHolder(index), act) in query.iter_mut() {
        let player = battle
            .players
            .iter()
            .find(|player| player.hero.id == selected.id)
            .unwrap();
        let Some((active, _)) = player.cards_reserved.get(*index) else {
            continue;
        };
        if *active {
            *color = BackgroundColor(match act {
                Interaction::None => Color::NONE,
                Interaction::Hovered => Color::rgba(1.0, 1.0, 1.0, 0.1),
                Interaction::Pressed => {
                    battle.buy_card(&selected.id, *index);
                    Color::rgba(0.0, 0.0, 0.0, 0.1)
                }
            });
        } else {
            *color = Color::GRAY.with_a(0.5).into();
        }
    }
}

#[derive(Component)]
struct CardHeader;

fn init_card_header(mut commands: Commands, query: Query<Entity, Added<CardHeader>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Percent(20.0),
                ..Default::default()
            },
            // background_color: DCOLOR,
            ..Default::default()
        });
    }
}

#[derive(Component)]
struct CardLevels(usize, u8, u8);

fn init_card_levels(
    mut commands: Commands,
    query: Query<(Entity, &CardLevels), Added<CardLevels>>,
) {
    for (entity, CardLevels(index, lvl_cur, lvl_max)) in query.iter() {
        let gap = 10.0;
        let level_width = (CARD_WIDTH - gap * (*lvl_max + 1) as f32) as f32 / *lvl_max as f32;
        let level_height = level_width + gap * 2.0;
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(gap),
                    padding: UiRect::all(Val::Px(gap)),
                    width: Val::Percent(100.0),
                    height: Val::Px(level_height),
                    ..Default::default()
                },
                // background_color: DCOLOR,
                ..Default::default()
            })
            .with_children(|p| {
                println!("{lvl_cur} / {lvl_max}");
                for _ in 0..*lvl_cur {
                    p.spawn((NodeBundle::default(), CardLevelActive));
                }
                p.spawn((NodeBundle::default(), CardLevelBlink(*index, 0.0)));
                for _ in *lvl_cur + 1..*lvl_max {
                    p.spawn((NodeBundle::default(), CardLevelInactive));
                }
            });
    }
}

#[derive(Component)]
struct CardLevelActive;

fn init_card_level_active(mut commands: Commands, query: Query<Entity, Added<CardLevelActive>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: Color::GOLD.with_a(0.1).into(),
            ..Default::default()
        });
    }
}

#[derive(Component)]
struct CardLevelInactive;

fn init_card_level_inactive(
    mut commands: Commands,
    query: Query<Entity, Added<CardLevelInactive>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: Color::BLACK.into(),
            ..Default::default()
        });
    }
}

#[derive(Component)]
struct CardLevelBlink(usize, f32);

fn init_card_level_blink(mut commands: Commands, query: Query<Entity, Added<CardLevelBlink>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: Color::GOLD.with_a(0.1).into(),
            ..Default::default()
        });
    }
}

fn update_card_level_blink(
    mut query: Query<(&mut CardLevelBlink, &mut BackgroundColor)>,
    battle: Res<BattleResource>,
    selected: Res<HeroSelected>,
    time: Res<Time>,
) {
    let color1 = Color::BLACK.rgba_to_vec4();
    let color2 = Color::GOLD.with_a(0.1).rgba_to_vec4();

    for (mut blink, mut color) in query.iter_mut() {
        let player = battle
            .players
            .iter()
            .find(|player| player.hero.id == selected.id)
            .unwrap();
        let Some((active, _)) = player.cards_reserved.get(blink.0) else {
            continue;
        };

        let c = color1 + (color2 - color1) * 0.5 * (1.0 + (blink.1 * TAU).cos());
        *color = Color::rgba(c.x, c.y, c.z, c.w).into();

        if *active {
            blink.1 += time.delta_seconds();
        } else {
            blink.1 = 0.0;
        }
    }
}

#[derive(Component)]
struct CardName(&'static str);

fn init_card_name(mut commands: Commands, query: Query<(Entity, &CardName), Added<CardName>>) {
    for (entity, name) in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Px(NAME_HEIGHT),
                    ..Default::default()
                },
                // background_color: DCOLOR,
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    name.0,
                    TextStyle {
                        font_size: 20.0,
                        ..Default::default()
                    },
                ));
            });
    }
}

#[derive(Component)]
struct CardDesc(&'static str);

fn init_card_desc(mut commands: Commands, query: Query<(Entity, &CardDesc), Added<CardDesc>>) {
    for (entity, desc) in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    padding: UiRect::horizontal(Val::Px(5.0)),
                    ..Default::default()
                },
                // background_color: DCOLOR,
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn(TextBundle::from_section(desc.0, TextStyle::default()));
            });
    }
}

#[derive(Component)]
struct CardFooter(u32);

fn init_card_footer(
    mut commands: Commands,
    query: Query<(Entity, &CardFooter), Added<CardFooter>>,
) {
    for (entity, footer) in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Px(FOOTER_HEIGHT),
                    ..Default::default()
                },
                background_color: DCOLOR,
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    format!("{}", footer.0),
                    TextStyle::default(),
                ));
            });
    }
}

#[derive(Component)]
struct CardsControls;

fn init_cards_controls(mut commands: Commands, query: Query<Entity, Added<CardsControls>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Px(CONTROLS_WIDTH),
                    height: Val::Px(HEIGHT),
                    ..Default::default()
                },
                // background_color: DCOLOR,
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((NodeBundle::default(), CardsControl("control 1".to_string())));
                p.spawn((NodeBundle::default(), CardsControl("control 2".to_string())));
                p.spawn((NodeBundle::default(), CardsControl("control 3".to_string())));
                p.spawn((NodeBundle::default(), CardsControl("control 4".to_string())));
            });
    }
}

#[derive(Component)]
struct CardsControl(String);

fn init_cards_control(
    mut commands: Commands,
    query: Query<(Entity, &CardsControl), Added<CardsControl>>,
) {
    for (entity, cards_control) in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(100.0),
                    height: Val::Percent(20.0),
                    margin: UiRect::vertical(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: DCOLOR,
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    &cards_control.0,
                    TextStyle {
                        font_size: 25.0,
                        ..Default::default()
                    },
                ));
            });
    }
}
