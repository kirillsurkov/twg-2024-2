use bevy::prelude::*;

use crate::component::game_timer::GameTimer;

use super::{LocalSchedule, UiAssets};

pub struct GameTimerPlugin;

impl Plugin for GameTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, (init_root, update_timer));
    }
}

#[derive(Component)]
pub struct GameTimerRoot;

fn init_root(
    mut commands: Commands,
    assets: Res<UiAssets>,
    query: Query<Entity, Added<GameTimerRoot>>,
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

fn update_timer(mut query: Query<&mut Text, With<GameTimerRoot>>, state: Res<GameTimer>) {
    for mut text in query.iter_mut() {
        let section = &mut text.sections[0];
        if state.red {
            section.style.color = Color::RED;
        } else {
            section.style.color = Color::WHITE;
        }
        section.value = format!("{:.0}", (state.max - state.value).ceil() - 1.0);
    }
}
