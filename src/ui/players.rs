use bevy::prelude::*;

use super::{LocalSchedule, DCOLOR};

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, init_root);
    }
}

#[derive(Component)]
pub struct PlayersRoot;

fn init_root(mut commands: Commands, query: Query<Entity, Added<PlayersRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(20.0),
                ..Default::default()
            },
            background_color: DCOLOR,
            ..Default::default()
        });
    }
}
