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
                init_side,
                init_header,
                init_body,
                init_footer,
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
pub struct ScreenSide;

fn init_side(mut commands: Commands, query: Query<Entity, Added<ScreenSide>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(25.0),
                height: Val::Percent(100.0),
                margin: UiRect::left(Val::Percent(-25.0)),
                ..Default::default()
            },
            background_color: DCOLOR,
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
                height: Val::Percent(10.0),
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
pub struct ScreenBody;

fn init_body(mut commands: Commands, query: Query<Entity, Added<ScreenBody>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                height: Val::Percent(85.0),
                ..Default::default()
            },
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
                height: Val::Percent(5.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: DCOLOR,
            ..Default::default()
        });
    }
}
