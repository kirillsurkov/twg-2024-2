use bevy::prelude::*;

use crate::scene::landing::HeroSelected;

use super::LocalSchedule;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, added.run_if(any_with_component::<Arena>));
    }
}

#[derive(Component)]
pub struct State {
    pub active: bool,
    pub changed: bool,
}

#[derive(Component)]
pub struct Arena {}

impl Arena {}

fn added(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Arena, &Children), Added<Arena>>,
    selected: Res<HeroSelected>,
) {
    for (entity, mut arena, children) in query.iter_mut() {}
}
