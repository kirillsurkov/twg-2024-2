use bevy::prelude::*;

use crate::battle_bridge::BattleResource;

use super::{LocalSchedule, UiAssets, DCOLOR};

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            init_root.run_if(resource_exists::<BattleResource>),
        );
    }
}

#[derive(Component)]
pub struct PlayersRoot;

fn init_root(
    mut commands: Commands,
    assets: Res<UiAssets>,
    query: Query<Entity, Added<PlayersRoot>>,
    battle: Res<BattleResource>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(20.0),
                    ..Default::default()
                },
                background_color: DCOLOR,
                ..Default::default()
            })
            .with_children(|p| {
                battle.players.iter().for_each(|player| {
                    p.spawn(TextBundle::from_section(
                        player.hero.name,
                        TextStyle {
                            font: assets.font_comic.clone_weak(),
                            font_size: 25.0,
                            ..Default::default()
                        },
                    ));
                })
            });
    }
}
