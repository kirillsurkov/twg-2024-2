use bevy::prelude::*;

use crate::scene::fight_arena;

use super::{LocalSchedule, UiAssets};

pub struct GameTimerPlugin;

impl Plugin for GameTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (init_root, update_timer).run_if(resource_exists::<fight_arena::State>),
        );
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

fn update_timer(
    mut query: Query<&mut Text, With<GameTimerRoot>>,
    arena_state: Res<fight_arena::State>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{:.0}", arena_state.timer_max - arena_state.timer);
    }
}
