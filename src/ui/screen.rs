use bevy::prelude::*;

use super::{LocalSchedule, DCOLOR};

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (
                init_root,
                init_main,
                init_header,
                init_body_root,
                init_body_top,
                init_body_bot,
                init_footer,
                init_bottom,
            ),
        );
    }
}

#[derive(Component)]
pub struct ScreenRoot;

fn init_root(mut commands: Commands, query: Query<Entity, Added<ScreenRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                margin: UiRect::all(Val::Px(0.0)),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

#[derive(Component)]
pub struct ScreenMain;

fn init_main(mut commands: Commands, query: Query<Entity, Added<ScreenMain>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

#[derive(Component)]
pub struct ScreenHeader;

fn init_header(mut commands: Commands, query: Query<Entity, Added<ScreenHeader>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            // background_color: DCOLOR,
            ..Default::default()
        });
    }
}

#[derive(Component)]
pub struct ScreenBodyRoot;

fn init_body_root(mut commands: Commands, query: Query<Entity, Added<ScreenBodyRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

#[derive(Component)]
pub struct ScreenBodyTop;

fn init_body_top(mut commands: Commands, query: Query<Entity, Added<ScreenBodyTop>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                height: Val::Percent(50.0),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

#[derive(Component)]
pub struct ScreenBodyBot;

fn init_body_bot(mut commands: Commands, query: Query<Entity, Added<ScreenBodyBot>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                height: Val::Percent(50.0),
                ..Default::default()
            },
            // background_color: Color::RED.into(),
            ..Default::default()
        });
    }
}

#[derive(Component)]
pub struct ScreenFooter;

fn init_footer(mut commands: Commands, query: Query<Entity, Added<ScreenFooter>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: DCOLOR,
            ..Default::default()
        });
    }
}

#[derive(Component)]
pub struct ScreenBottom;

fn init_bottom(mut commands: Commands, query: Query<Entity, Added<ScreenBottom>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Px(200.0),
                margin: UiRect::top(Val::Px(-200.0)),
                ..Default::default()
            },
            // background_color: DCOLOR,
            ..Default::default()
        });
    }
}
