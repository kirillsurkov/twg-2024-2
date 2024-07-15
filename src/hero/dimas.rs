use std::{
    f32::consts::{FRAC_PI_2, FRAC_PI_3, TAU},
    time::Duration,
};

use bevy::{gltf::Gltf, prelude::*};

use crate::{
    battle::modifier::Modifier,
    component::{
        arena,
        complex_anim_player::{self, Animations, ComplexAnimPart, ComplexAnimPlayer, Showoff},
        fight_state::FightState,
        model::Model,
        projectile::{Projectile, ProjectileConfig},
    },
    scene::{
        avatars::{self, AvatarLocation},
        Root,
    },
};

use super::{HeroId, LocalSchedule};

#[derive(Component)]
pub struct Dimas;

#[derive(Component)]
struct State {
    swiborg: Handle<Scene>,
}

#[derive(Component)]
struct Swiborg(u32);

#[derive(Component)]
struct Ready;

#[derive(Component)]
struct ModelReady;

#[derive(Component)]
struct SwiborgRing;

impl Plugin for Dimas {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (
                on_add,
                filter_animations,
                on_avatar,
                on_arena.run_if(resource_exists::<FightState>),
                swiborg_ring,
            ),
        );
    }
}

fn on_add(
    mut commands: Commands,
    model: Option<Res<Model<Dimas>>>,
    asset_server: Res<AssetServer>,
    assets_gltf: Res<Assets<Gltf>>,
    query: Query<Entity, (With<Dimas>, Without<Ready>)>,
    query_model: Query<Entity, (With<Dimas>, Without<ModelReady>)>,
    query_animation: Query<Entity, (With<Dimas>, With<ModelReady>)>,
    children: Query<&Parent>,
    anim_players: Query<Entity, With<AnimationPlayer>>,
) {
    if query.is_empty() {
        return;
    }

    let gltf = match model {
        Some(model) => match assets_gltf.get(&model.handle) {
            Some(gltf) => gltf,
            None => return,
        },
        None => {
            commands.insert_resource(Model::<Dimas>::new(
                asset_server.load("embedded://dimas.glb"),
            ));
            return;
        }
    };

    for entity in query_model.iter() {
        commands
            .entity(entity)
            .insert((
                ModelReady,
                State {
                    swiborg: asset_server.load("embedded://swiborg.glb#Scene0"),
                },
                ProjectileConfig {
                    offset: Vec3::new(0.0, 0.0, 0.0),
                    color: Color::ORANGE,
                    radius: 0.3,
                    model: None,
                },
            ))
            .with_children(|p| {
                p.spawn(SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    transform: Transform::from_scale(Vec3::splat(1.0)),
                    ..Default::default()
                });
                p.spawn((
                    SwiborgRing,
                    TransformBundle::default(),
                    VisibilityBundle::default(),
                ));
            });
    }

    for entity in query_animation.iter() {
        let mut entity = commands.entity(entity);
        entity.insert(Ready);
        for anim_player in anim_players.iter() {
            for parent in children.iter_ancestors(anim_player) {
                if parent == entity.id() {
                    entity.insert((
                        ComplexAnimPlayer::new(anim_player)
                            .with_idle("idle_track")
                            .with_attack("attack_track", 40)
                            .with_win("win_track")
                            .with_lose("lose_track")
                            .with_showoff(Showoff::new(vec![ComplexAnimPart {
                                name: "legs_sit_track".to_string(),
                                repeat: 1,
                                speed: 2.0,
                                wait: Duration::from_millis(0),
                            }])),
                        Animations::new(gltf.named_animations.clone()),
                    ));
                }
            }
        }
    }
}

fn filter_animations(
    mut query: Query<&Animations, With<Dimas>>,
    mut named: Query<(&Name, &mut Visibility)>,
) {
}

fn on_avatar(mut query: Query<(&mut ComplexAnimPlayer, &mut avatars::HeroState), With<Dimas>>) {
    for (mut anim_player, mut state) in query.iter_mut() {
        let origin = Vec3::new(0.6, 2.2, 1.8);
        let target = origin.xy().extend(0.0);
        state.camera = match state.location {
            AvatarLocation::Thumbnail => {
                Transform::from_translation(origin).looking_at(target, Vec3::Y)
            }
            _ => {
                anim_player.play(false, complex_anim_player::State::Idle);
                Transform::from_translation(
                    target + Quat::from_rotation_y(FRAC_PI_3) * (origin - target),
                )
                .looking_at(target, Vec3::Y)
            }
        }
    }
}

fn on_arena(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &arena::HeroState, &State, &HeroId), With<Dimas>>,
    rings: Query<(Entity, &Parent, Option<&Children>), With<SwiborgRing>>,
    swiborgs: Query<&Swiborg>,
    transforms: Query<&GlobalTransform>,
    root: Query<Entity, With<Root>>,
) {
    let Ok(root) = root.get_single() else {
        return;
    };

    for (entity, mut transform, arena_state, state, id) in query.iter_mut() {
        transform.rotation = Quat::from_rotation_y(-FRAC_PI_2);

        let (ring, _, children) = rings.iter().find(|(_, p, _)| p.get() == entity).unwrap();

        for modifier in &arena_state.modifiers {
            match modifier {
                Modifier::SpawnSwiborg(i) => {
                    commands.entity(ring).with_children(|p| {
                        p.spawn((
                            Swiborg(*i),
                            TransformBundle {
                                local: Transform::from_translation(Vec3::new(0.0, 3.0, 0.0)),
                                ..Default::default()
                            },
                            VisibilityBundle::default(),
                        ))
                        .with_children(|p| {
                            p.spawn((
                                id.clone(),
                                Projectile::new(p.parent_entity(), None, 0.5),
                                ProjectileConfig {
                                    offset: Vec3::ZERO,
                                    color: Color::LIME_GREEN,
                                    radius: 0.25,
                                    model: Some(state.swiborg.clone_weak()),
                                },
                            ));
                        });
                    });
                }
                Modifier::ShootSwiborg(i) => {
                    let swiborg = children
                        .unwrap()
                        .iter()
                        .find(|c| swiborgs.get(**c).unwrap().0 == *i)
                        .unwrap();
                    let offset = transforms.get(*swiborg).unwrap().translation();
                    commands.entity(*swiborg).despawn_recursive();
                    commands.entity(root).with_children(|p| {
                        p.spawn((
                            id.clone(),
                            Projectile::new(root, Some(arena_state.enemy), 0.5),
                            ProjectileConfig {
                                offset,
                                color: Color::LIME_GREEN,
                                radius: 0.25,
                                model: Some(state.swiborg.clone_weak()),
                            },
                        ));
                    });
                }
                _ => {}
            }
        }
    }
}

fn swiborg_ring(
    mut transforms: Query<&mut Transform>,
    query: Query<(Entity, &SwiborgRing, &Children)>,
    time: Res<Time>,
) {
    for (entity, ring, children) in query.iter() {
        let step = TAU / children.len() as f32;
        for (i, child) in children.iter().enumerate() {
            let ang = i as f32 * step;
            let x = ang.cos() * 2.0;
            let y = ang.sin() * 2.0;
            let mut transform = transforms.get_mut(*child).unwrap();
            let delta = Vec3::new(x, 3.0, y) - transform.translation;
            transform.translation += delta * time.delta_seconds() * 5.0;
        }
        transforms
            .get_mut(entity)
            .unwrap()
            .rotate_y(time.delta_seconds());
    }
}
