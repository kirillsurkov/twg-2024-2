use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use crate::{
    battle::{fight::Owner, modifier::Modifier, RoundCapture},
    battle_bridge::RoundCaptureResource,
    hero::HeroId,
    scene::landing::HeroWatch,
};

use super::{game_timer::GameTimer, LocalSchedule};

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (init, update).run_if(any_with_component::<Arena>),
        );
    }
}

#[derive(Component)]
pub struct HeroState {
    pub modifiers: Vec<Modifier>,
    pub enemy: Entity,
}

#[derive(Component)]
pub struct Arena {}

impl Arena {}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &mut Arena, &Children), Added<Arena>>,
    capture: Res<RoundCaptureResource>,
    hero_ids: Query<&HeroId>,
    with_parent: Query<&Parent>,
) {
    for (entity, mut arena, children) in query.iter_mut() {
        for capture in &capture.0 {
            for hero in children.iter() {
                let id = &hero_ids.get(*hero).unwrap().0;

                let transform = match capture {
                    RoundCapture::Fight {
                        player1, player2, ..
                    } => {
                        let fighter = if id == player1 {
                            Owner::Fighter1
                        } else if id == player2 {
                            Owner::Fighter2
                        } else {
                            continue;
                        };

                        let x = match fighter {
                            Owner::Fighter1 => -4.0,
                            Owner::Fighter2 => 4.0,
                        };

                        let rotation = match fighter {
                            Owner::Fighter1 => Quat::from_rotation_y(FRAC_PI_2),
                            Owner::Fighter2 => Quat::from_rotation_y(-FRAC_PI_2),
                        };

                        Transform::from_translation(Vec3::new(x, 0.0, 0.0)).with_rotation(rotation)
                    }
                    RoundCapture::Skip(player) => {
                        if id != player {
                            continue;
                        }
                        Transform::default()
                    }
                };

                let mut parent = commands.entity(with_parent.get(*hero).unwrap().get());
                let mut hero_node = Entity::PLACEHOLDER;
                parent.with_children(|p| {
                    hero_node = p
                        .spawn((
                            TransformBundle {
                                local: transform,
                                ..Default::default()
                            },
                            VisibilityBundle::default(),
                        ))
                        .id();
                });

                commands
                    .entity(*hero)
                    .insert((
                        HeroState {
                            modifiers: vec![],
                            enemy: Entity::PLACEHOLDER,
                        },
                        TransformBundle::default(),
                        VisibilityBundle {
                            visibility: Visibility::Hidden,
                            ..Default::default()
                        },
                    ))
                    .set_parent(hero_node);
            }
        }

        commands
            .entity(entity)
            .insert((
                TransformBundle {
                    local: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                VisibilityBundle::default(),
            ))
            .with_children(|p| {
                p.spawn(PbrBundle {
                    mesh: meshes.add(Plane3d {
                        normal: Direction3d::Y,
                    }),
                    material: materials.add(StandardMaterial::default()),
                    transform: Transform::from_scale(Vec3::splat(10000.0)),
                    ..Default::default()
                });
            });
    }
}

fn update(
    mut commands: Commands,
    mut game_timer: ResMut<GameTimer>,
    round: Res<RoundCaptureResource>,
    watch: Res<HeroWatch>,
    time: Res<Time>,
    query: Query<(Entity, &HeroId), With<HeroState>>,
) {
    if round.0.iter().fold(true, |acc, c| {
        acc && match c {
            RoundCapture::Fight { fight_capture, .. } => {
                game_timer.value >= fight_capture.duration()
            }
            RoundCapture::Skip(_) => true,
        }
    }) {
        game_timer.fired = true;
    }

    for (entity, id) in query.iter() {
        let round = round.by_player(&id.0).unwrap();

        let show = match round {
            RoundCapture::Fight {
                player1, player2, ..
            } => *player1 == watch.id || *player2 == watch.id,
            RoundCapture::Skip(player) => *player == watch.id,
        };

        if show {
            commands.entity(entity).insert(Visibility::Inherited);
        } else {
            commands.entity(entity).insert(Visibility::Hidden);
        }

        let RoundCapture::Fight {
            player1,
            player2,
            fight_capture,
            ..
        } = round
        else {
            continue;
        };

        if game_timer.red {
            continue;
        }

        let owner = if *player1 == id.0 {
            Owner::Fighter1
        } else {
            Owner::Fighter2
        };

        let enemy_id = match owner {
            Owner::Fighter1 => *player2,
            Owner::Fighter2 => *player1,
        };

        let fight = fight_capture;
        let modifiers = if let Some(state) =
            fight.state(game_timer.value, game_timer.value + time.delta_seconds())
        {
            state
                .modifiers
                .into_iter()
                .filter_map(|(o, m)| if o == owner { Some(m.modifier) } else { None })
                .collect()
        } else {
            vec![]
        };

        commands.entity(entity).insert(HeroState {
            enemy: query.iter().find(|(_, id)| id.0 == enemy_id).unwrap().0,
            modifiers,
        });
    }
}
