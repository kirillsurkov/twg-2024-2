use bevy::prelude::*;

use crate::{
    battle_bridge::{BattleResource, RoundCaptureResource},
    hero::HeroId,
    scene::landing::{HeroSelected, HeroWatch},
};

use super::{LocalSchedule, UiAssets, DCOLOR};

pub struct StatsPlugin;

const HEIGHT: f32 = 50.0;

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
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::left(Val::Auto),
                    width: Val::Px(200.0),
                    ..Default::default()
                },
                background_color: DCOLOR,
                ..Default::default()
            })
            .with_children(|p| {});
    }
}
