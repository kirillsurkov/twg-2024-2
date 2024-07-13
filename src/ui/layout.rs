use bevy::prelude::*;

use super::{LocalSchedule, DCOLOR};

pub struct LayoutPlugin;

impl Plugin for LayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, (init_vbox, init_hbox));
    }
}

#[derive(Component)]
pub struct VBox;

fn init_vbox(mut commands: Commands, query: Query<Entity, Added<VBox>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
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
pub struct HBox;

fn init_hbox(mut commands: Commands, query: Query<Entity, Added<HBox>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: DCOLOR,
            ..Default::default()
        });
    }
}
