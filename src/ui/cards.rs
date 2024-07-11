use bevy::prelude::*;

use super::{LocalSchedule, DCOLOR};

pub struct CardsPlugin;

impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (
                init_root,
                init_cards_holder,
                init_card_holder,
                update_card_holder,
                init_card_header,
                init_card_levels,
                init_card_level_active,
                init_card_level_inactive,
                init_card_desc,
                init_card_footer,
                init_cards_controls,
                init_cards_control,
            ),
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
                    margin: UiRect::top(Val::Auto),
                    width: Val::Percent(100.0),
                    height: Val::Percent(40.0),
                    ..Default::default()
                },
                background_color: DCOLOR,
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((NodeBundle::default(), CardsHolder));
                p.spawn((NodeBundle::default(), CardsControls));
            });
    }
}

#[derive(Component)]
struct CardsHolder;

fn init_cards_holder(mut commands: Commands, query: Query<Entity, Added<CardsHolder>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::Center,
                    width: Val::Percent(65.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                background_color: DCOLOR,
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((NodeBundle::default(), CardHolder))
                    .with_children(|p| {
                        p.spawn((NodeBundle::default(), CardHeader));
                        p.spawn((NodeBundle::default(), CardLevels));
                        p.spawn((
                            CardDesc,
                            TextBundle::from_section("Card 1", TextStyle::default()),
                        ));
                        p.spawn((NodeBundle::default(), CardFooter));
                    });
                p.spawn((NodeBundle::default(), CardHolder))
                    .with_children(|p| {
                        p.spawn((NodeBundle::default(), CardHeader));
                        p.spawn((NodeBundle::default(), CardLevels));
                        p.spawn((
                            CardDesc,
                            TextBundle::from_section("Card 2", TextStyle::default()),
                        ));
                        p.spawn((NodeBundle::default(), CardFooter));
                    });
                p.spawn((NodeBundle::default(), CardHolder))
                    .with_children(|p| {
                        p.spawn((NodeBundle::default(), CardHeader));
                        p.spawn((NodeBundle::default(), CardLevels));
                        p.spawn((
                            CardDesc,
                            TextBundle::from_section("Card 3", TextStyle::default()),
                        ));
                        p.spawn((NodeBundle::default(), CardFooter));
                    });
            });
    }
}

#[derive(Component)]
struct CardHolder;

fn init_card_holder(mut commands: Commands, query: Query<Entity, Added<CardHolder>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(ButtonBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(15.0),
                height: Val::Percent(100.0),
                margin: UiRect::right(Val::Percent(2.0)),
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        });
    }
}

fn update_card_holder(mut query: Query<(&mut BackgroundColor, &Interaction), With<CardHolder>>) {
    for (mut color, act) in query.iter_mut() {
        *color = BackgroundColor(match act {
            Interaction::None => Color::NONE.into(),
            Interaction::Hovered => Color::rgba(1.0, 1.0, 1.0, 0.1),
            Interaction::Pressed => Color::rgba(0.0, 0.0, 0.0, 0.1),
        });
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
            background_color: DCOLOR,
            ..Default::default()
        });
    }
}

#[derive(Component)]
struct CardLevels;

fn init_card_levels(mut commands: Commands, query: Query<Entity, Added<CardLevels>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                ..Default::default()
            },
            background_color: DCOLOR,
            ..Default::default()
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
            background_color: DCOLOR,
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
            background_color: DCOLOR,
            ..Default::default()
        });
    }
}

#[derive(Component)]
struct CardDesc;

fn init_card_desc(mut commands: Commands, query: Query<Entity, Added<CardDesc>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(50.0),
                ..Default::default()
            },
            background_color: DCOLOR,
            ..Default::default()
        });
    }
}

#[derive(Component)]
struct CardFooter;

fn init_card_footer(mut commands: Commands, query: Query<Entity, Added<CardFooter>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(20.0),
                margin: UiRect::new(
                    Val::Px(0.0),
                    Val::Px(0.0),
                    Val::Percent(5.0),
                    Val::Percent(5.0),
                ),
                ..Default::default()
            },
            background_color: DCOLOR,
            ..Default::default()
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
                    width: Val::Percent(10.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                background_color: DCOLOR,
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
