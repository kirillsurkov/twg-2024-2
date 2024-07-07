use bevy::prelude::*;

use crate::scene::landing::HeroSelected;

use super::LocalSchedule;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, added);
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
    selected: Option<Res<HeroSelected>>,
    named: Query<&Name>,
) {
    for (entity, mut arena, children) in query.iter_mut() {}
}
