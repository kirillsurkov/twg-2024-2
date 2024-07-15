use std::{
    f32::consts::{FRAC_PI_2, FRAC_PI_6, PI},
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
pub struct Duck;

#[derive(Component)]
struct State {
    duck: Handle<Scene>,
}

#[derive(Component)]
pub struct Ready;

#[derive(Component)]
pub struct ModelReady;

impl Plugin for Duck {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (
                on_add,
                filter_animations,
                on_avatar,
                on_arena.run_if(resource_exists::<FightState>),
            ),
        );
    }
}

fn on_add(
    mut commands: Commands,
    model: Option<Res<Model<Duck>>>,
    asset_server: Res<AssetServer>,
    assets_gltf: Res<Assets<Gltf>>,
    query: Query<Entity, (With<Duck>, Without<Ready>)>,
    query_model: Query<Entity, (With<Duck>, Without<ModelReady>)>,
    query_animation: Query<Entity, (With<Duck>, With<ModelReady>)>,
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
            commands.insert_resource(Model::<Duck>::new(asset_server.load("embedded://duck.glb")));
            return;
        }
    };

    for entity in query_model.iter() {
        commands
            .entity(entity)
            .insert((
                ModelReady,
                State {
                    duck: asset_server.load("embedded://duck.glb#Scene0"),
                },
                ProjectileConfig {
                    transform: Transform::from_translation(Vec3::new(0.0, 1.086545, 0.97346)),
                    color: Color::YELLOW,
                    radius: 0.1,
                    ..Default::default()
                },
            ))
            .with_children(|p| {
                p.spawn(SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    transform: Transform::from_scale(Vec3::splat(0.5)),
                    ..Default::default()
                });
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
                                name: "flapping_track".to_string(),
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
    mut query: Query<&Animations, With<Duck>>,
    mut named: Query<(&Name, &mut Visibility)>,
) {
}

fn on_avatar(mut query: Query<(&mut ComplexAnimPlayer, &mut avatars::HeroState), With<Duck>>) {
    for (mut anim_player, mut state) in query.iter_mut() {
        let origin = Vec3::new(0.0, 1.2, 1.5);
        let target = origin.xy().extend(0.0);
        state.camera = match state.location {
            AvatarLocation::Thumbnail => {
                Transform::from_translation(origin).looking_at(target, Vec3::Y)
            }
            _ => {
                let origin = Vec3::new(0.0, 1.2, 2.5);
                let target = Vec3::new(0.0, 0.6, 0.0);
                anim_player.play(false, complex_anim_player::State::Idle);
                Transform::from_translation(
                    target + Quat::from_rotation_y(-FRAC_PI_6) * (origin - target),
                )
                .looking_at(target, Vec3::Y)
            }
        }
    }
}

fn on_arena(
    mut commands: Commands,
    query: Query<(Entity, &arena::HeroState, &State, &HeroId), With<Duck>>,
    transforms: Query<&GlobalTransform>,
    root: Query<Entity, With<Root>>,
) {
    let Ok(root) = root.get_single() else {
        return;
    };

    for (entity, arena_state, state, id) in query.iter() {
        for modifier in &arena_state.modifiers {
            match modifier {
                Modifier::ShootDuck => {
                    let offset = transforms.get(entity).unwrap().translation();
                    commands.entity(root).with_children(|p| {
                        p.spawn((
                            id.clone(),
                            Projectile::new(root, Some(arena_state.enemy), 0.5),
                            ProjectileConfig {
                                transform: Transform::from_translation(offset)
                                    .with_rotation(Quat::from_rotation_y(PI))
                                    .with_scale(Vec3::splat(0.5)),
                                color: Color::DARK_GREEN,
                                radius: 0.5,
                                model: Some(state.duck.clone_weak()),
                                model_transform: Transform::from_translation(Vec3::new(
                                    0.0, -1.25, 0.0,
                                )),
                            },
                        ));
                    });
                }
                _ => {}
            }
        }
    }
}
