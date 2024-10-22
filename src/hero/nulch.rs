use std::{f32::consts::FRAC_PI_6, time::Duration};

use bevy::{audio::{PlaybackMode, Volume}, gltf::Gltf, prelude::*, render::view::visibility};

use crate::{
    battle::modifier::Modifier,
    component::{
        arena,
        beam::Beam,
        complex_anim_player::{self, Animations, ComplexAnimPart, ComplexAnimPlayer, Showoff},
        fight_state::FightState,
        model::Model,
        projectile::ProjectileConfig,
    },
    scene::{
        avatars::{self, AvatarLocation},
        Root,
    }, MASTER_VOLUME,
};

use super::{HeroId, LocalSchedule};

#[derive(Component)]
pub struct Nulch;

#[derive(Component)]
pub struct Ready;

#[derive(Component)]
pub struct ModelReady;

impl Plugin for Nulch {
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
    model: Option<Res<Model<Nulch>>>,
    asset_server: Res<AssetServer>,
    assets_gltf: Res<Assets<Gltf>>,
    query: Query<Entity, (With<Nulch>, Without<Ready>)>,
    query_model: Query<Entity, (With<Nulch>, Without<ModelReady>)>,
    query_animation: Query<Entity, (With<Nulch>, With<ModelReady>)>,
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
            commands.insert_resource(Model::<Nulch>::new(
                asset_server.load("embedded://nulch.glb"),
            ));
            return;
        }
    };

    for entity in query_model.iter() {
        commands
            .entity(entity)
            .insert(ModelReady)
            .with_children(|p| {
                p.spawn(SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    transform: Transform::from_scale(Vec3::splat(0.1)),
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
                                name: "drink_track".to_string(),
                                repeat: 1,
                                speed: 1.0,
                                wait: Duration::from_millis(0),
                            }])),
                        Animations::new(gltf.named_animations.clone()),
                        ProjectileConfig {
                            transform: Transform::from_translation(Vec3::new(
                                0.0, 2.44231, -2.36796,
                            )),
                            color: Color::CYAN,
                            radius: 0.25,
                            ..Default::default()
                        },
                    ));
                }
            }
        }
    }
}

fn filter_animations(
    mut query: Query<&Animations, With<Nulch>>,
    mut named: Query<(&Name, &mut Visibility)>,
) {
    for anims in query.iter_mut() {
        let current_anim = anims.current();

        for (name, mut visibility) in named.iter_mut() {
            match name.as_str() {
                "teacup" => {
                    *visibility = match current_anim.as_str() {
                        "attack_track" => Visibility::Hidden,
                        _ => Visibility::Inherited,
                    }
                }
                _ => {}
            }
        }
    }
}

fn on_avatar(mut query: Query<(&mut ComplexAnimPlayer, &mut avatars::HeroState), With<Nulch>>) {
    for (mut anim_player, mut state) in query.iter_mut() {
        let origin = Vec3::new(0.0, 2.3, 0.75);
        let target = origin.xy().extend(0.0);
        state.camera = match state.location {
            AvatarLocation::Thumbnail => {
                Transform::from_translation(origin).looking_at(target, Vec3::Y)
            }
            _ => {
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
    asset_server: Res<AssetServer>,
    query: Query<
        (Entity, &arena::HeroState, &HeroId, &InheritedVisibility),
        (With<Nulch>, With<Ready>),
    >,
    transforms: Query<&GlobalTransform>,
    root: Query<Entity, With<Root>>,
) {
    let Ok(root) = root.get_single() else {
        return;
    };

    for (entity, arena_state, id, visibility) in query.iter() {
        for modifier in &arena_state.modifiers {
            match modifier {
                Modifier::NormalAttack => {
                    if visibility.get() {
                        commands.entity(entity).with_children(|p| {
                            p.spawn(AudioBundle {
                                source: asset_server.load("embedded://shoot5.ogg"),
                                settings: PlaybackSettings {
                                    // mode: PlaybackMode::Despawn,
                                    volume: Volume::new(MASTER_VOLUME),
                                    ..Default::default()
                                },
                            });
                        });
                    }
                }
                Modifier::ShootHealBeam => {
                    let offset = transforms.get(entity).unwrap().translation();
                    commands.entity(root).with_children(|p| {
                        p.spawn((
                            id.clone(),
                            Beam::new(1.0)
                                .with_transform(Transform::from_translation(offset))
                                .with_color(Color::LIME_GREEN * 2.0),
                        ));
                    });
                }
                Modifier::ShootDamageBeam => {
                    let offset = transforms.get(arena_state.enemy).unwrap().translation();
                    commands.entity(root).with_children(|p| {
                        p.spawn((
                            id.clone(),
                            Beam::new(1.0)
                                .with_transform(Transform::from_translation(offset))
                                .with_color(Color::RED * 2.0),
                        ));
                    });
                }
                _ => {}
            }
        }
    }
}
