use bevy::prelude::*;

use crate::battle_bridge::BattleResource;

use super::{LocalSchedule, DCOLOR};

pub struct StatsPlugin;

pub const WIDTH: f32 = 200.0;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (init_stats_root,).run_if(resource_exists::<BattleResource>),
        );
    }
}

#[derive(Component)]
pub struct StatsRoot;

fn init_stats_root(mut commands: Commands, query: Query<Entity, Added<StatsRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                margin: UiRect::left(Val::Auto),
                width: Val::Px(WIDTH),
                ..Default::default()
            },
            background_color: DCOLOR,
            ..Default::default()
        });
    }
}
