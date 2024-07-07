use bevy::prelude::*;

use crate::scene::landing::HeroSelected;

use super::LocalSchedule;

pub struct LandPlugin;

impl Plugin for LandPlugin {
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
pub struct Land {}

impl Land {}

fn added(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Land, &Children), Added<Land>>,
    selected: Option<Res<HeroSelected>>,
    named: Query<&Name>,
) {
    for (entity, mut land, children) in query.iter_mut() {
        let selected = selected.as_ref().unwrap();

        let mut children = children.iter().map(|e| *e).collect::<Vec<_>>();
        children.sort_unstable_by_key(|c| named.get(*c).unwrap().as_str() != selected.name);

        for (i, child) in children.into_iter().enumerate() {
            let (x, y) = match i {
                0 => (0.0, 0.0),
                1 => (-5.0, -5.0),
                2 => (5.0, -5.0),
                3 => (0.0, -5.0),
                4 => (0.0, -10.0),
                _ => unreachable!(),
            };

            let transform = TransformBundle {
                local: Transform::from_translation(Vec3::new(x, 0.0, y)),
                ..Default::default()
            };
            commands.entity(child).insert((
                transform,
                State {
                    active: i == 0,
                    changed: true,
                },
                VisibilityBundle::default(),
            ));
        }

        commands.entity(entity).insert((
            TransformBundle {
                local: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            VisibilityBundle::default(),
        ));
    }
}
