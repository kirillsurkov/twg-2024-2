use bevy::prelude::*;

use crate::{hero::Hero, scene::landing::HeroSelected};

use super::LocalSchedule;

pub struct LandPlugin;

impl Plugin for LandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, (added, show));
    }
}

#[derive(Component)]
pub struct State {
    pub active: bool,
    pub changed: bool,
}

#[derive(Component, Default)]
pub struct Land {
    index: usize,
    timer: f32,
    ready: bool,
}

impl Land {
    pub fn ready(&self) -> bool {
        self.ready
    }
}

fn added(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Land, &Children), Added<Land>>,
    selected: Option<Res<HeroSelected>>,
    heroes: Query<&Hero>,
) {
    for (entity, mut land, children) in query.iter_mut() {
        let selected = selected.as_ref().unwrap();

        let mut children = children.iter().map(|e| *e).collect::<Vec<_>>();
        children.sort_unstable_by_key(|c| heroes.get(*c).unwrap().id != selected.id);

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
                VisibilityBundle {
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
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

fn show(
    mut query: Query<(&mut Land, &Children)>,
    mut visibilities: Query<&mut Visibility>,
    selected: Option<Res<HeroSelected>>,
    heroes: Query<&Hero>,
    time: Res<Time>,
) {
    for (mut land, children) in query.iter_mut() {
        let selected = selected.as_ref().unwrap();
        if land.timer >= 0.5 {
            if land.index < children.len() {
                let mut children = children.iter().map(|e| *e).collect::<Vec<_>>();
                children.sort_by_key(|c| heroes.get(*c).unwrap().id == selected.id);
                *visibilities.get_mut(children[land.index]).unwrap() = Visibility::Visible;
                land.timer = 0.0;
                land.index += 1;
            } else {
                land.ready = true;
            }
        } else {
            land.timer += time.delta_seconds();
        }
    }
}
