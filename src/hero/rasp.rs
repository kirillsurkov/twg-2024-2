use std::{
    f32::consts::{FRAC_PI_6, SQRT_2},
    time::Duration,
};

use bevy::{gltf::Gltf, prelude::*, utils::HashMap};

use crate::{
    battle::{ability::fire_cube::CUBE_SIDE, modifier::Modifier},
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
pub struct Rasp;

#[derive(Component)]
struct State {
    fires: HashMap<u32, Entity>,
}

#[derive(Component)]
struct Fire(u32);

#[derive(Component)]
struct FireCube;

#[derive(Component)]
struct Ready;

#[derive(Component)]
struct ModelReady;

impl Plugin for Rasp {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (
                on_add,
                filter_animations,
                on_avatar,
                on_arena.run_if(resource_exists::<FightState>),
                fire_cube,
            ),
        );
    }
}

fn on_add(
    mut commands: Commands,
    model: Option<Res<Model<Rasp>>>,
    asset_server: Res<AssetServer>,
    assets_gltf: Res<Assets<Gltf>>,
    query: Query<Entity, (With<Rasp>, Without<Ready>)>,
    query_model: Query<Entity, (With<Rasp>, Without<ModelReady>)>,
    query_animation: Query<Entity, (With<Rasp>, With<ModelReady>)>,
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
            commands.insert_resource(Model::<Rasp>::new(asset_server.load("embedded://rasp.glb")));
            return;
        }
    };

    for entity in query_model.iter() {
        commands
            .entity(entity)
            .insert((
                ModelReady,
                State {
                    fires: HashMap::new(),
                },
                ProjectileConfig {
                    transform: Transform::from_translation(Vec3::new(0.0359386, 1.85802, 1.39652)),
                    color: Color::PINK,
                    radius: 0.05,
                    ..Default::default()
                },
            ))
            .with_children(|p| {
                p.spawn(SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    transform: Transform::from_scale(Vec3::splat(0.1)),
                    ..Default::default()
                });
                p.spawn((
                    FireCube,
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
                            .with_attack("shoot_track", 20)
                            .with_win("win_track")
                            .with_lose("lose_track")
                            .with_showoff(Showoff::new(vec![
                                ComplexAnimPart {
                                    name: "start_shoot_track".to_string(),
                                    repeat: 1,
                                    speed: 1.0,
                                    wait: Duration::from_millis(1000),
                                },
                                ComplexAnimPart {
                                    name: "shoot_track".to_string(),
                                    repeat: 3,
                                    speed: 3.0,
                                    wait: Duration::from_millis(500),
                                },
                            ])),
                        Animations::new(gltf.named_animations.clone()),
                    ));
                }
            }
        }
    }
}

fn filter_animations(
    mut query: Query<&Animations, With<Rasp>>,
    mut named: Query<(&Name, &mut Visibility)>,
) {
    for anims in query.iter_mut() {
        let current_anim = anims.current();

        for (name, mut visibility) in named.iter_mut() {
            if name.as_str() == "pistol" {
                *visibility = match current_anim.as_str() {
                    "start_shoot_track" | "shoot_track" => Visibility::Inherited,
                    _ => Visibility::Hidden,
                };
            }
        }
    }
}

fn on_avatar(mut query: Query<(&mut ComplexAnimPlayer, &mut avatars::HeroState), With<Rasp>>) {
    for (mut anim_player, mut state) in query.iter_mut() {
        let origin = Vec3::new(-0.15, 2.1, 1.0);
        let target = origin.xy().extend(0.0);
        state.camera = match state.location {
            AvatarLocation::Thumbnail => {
                Transform::from_translation(origin).looking_at(target, Vec3::Y)
            }
            _ => {
                let origin = Vec3::new(0.0, 2.2, 1.0);
                let target = origin.xy().extend(0.2);
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
    mut query: Query<(Entity, &arena::HeroState, &mut State, &HeroId), With<Rasp>>,
    cubes: Query<(Entity, &Parent), With<FireCube>>,
    transforms: Query<&GlobalTransform>,
    root: Query<Entity, With<Root>>,
) {
    let Ok(root) = root.get_single() else {
        return;
    };

    let projectile_config = ProjectileConfig {
        color: Color::ORANGE_RED,
        color_end: Color::BLACK,
        radius: 0.2,
        particles: 1024,
        ..Default::default()
    };

    for (entity, arena_state, mut state, id) in query.iter_mut() {
        let (cube, _) = cubes.iter().find(|(_, p)| p.get() == entity).unwrap();

        for modifier in &arena_state.modifiers {
            match modifier {
                Modifier::SpawnFireCube(i) => {
                    commands.entity(cube).with_children(|p| {
                        let id = p
                            .spawn((
                                Fire(*i),
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
                                    projectile_config.clone(),
                                ));
                            })
                            .id();
                        state.fires.insert(*i, id);
                    });
                }
                Modifier::ShootFireCube(i) => {
                    let fire = state.fires.remove(i).unwrap();
                    let offset = transforms.get(fire).unwrap().translation();
                    commands.entity(fire).despawn_recursive();
                    commands.entity(root).with_children(|p| {
                        let mut config = projectile_config.clone();
                        config.transform = Transform::from_translation(offset);
                        p.spawn((
                            id.clone(),
                            Projectile::new(root, Some(arena_state.enemy), 0.5),
                            config,
                        ));
                    });
                }
                _ => {}
            }
        }
    }
}

fn fire_cube(
    mut fires: Query<(&Fire, &Parent, &mut Transform), Added<Fire>>,
    mut query: Query<(Entity, &Parent, &mut Transform), (With<FireCube>, Without<Fire>)>,
    transforms: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    let side = 3.0;

    for (entity, parent, mut transform) in query.iter_mut() {
        for (fire, parent, mut transform) in fires.iter_mut() {
            if parent.get() != entity {
                continue;
            }

            transform.translation = (side / CUBE_SIDE as f32)
                * (Vec3::new(
                    ((fire.0 / CUBE_SIDE.pow(0)) % CUBE_SIDE) as f32,
                    ((fire.0 / CUBE_SIDE.pow(1)) % CUBE_SIDE) as f32,
                    ((fire.0 / CUBE_SIDE.pow(2)) % CUBE_SIDE) as f32,
                ) - ((CUBE_SIDE - 1) as f32 / 2.0));
        }

        transform.rotate_x(time.delta_seconds());
        transform.rotate_y(time.delta_seconds());
        transform.rotate_z(time.delta_seconds());

        let gtransform = transforms.get(parent.get()).unwrap();

        let global_pos = gtransform.translation() + Vec3::new(0.0, side * 0.5 * SQRT_2, -4.0)
            - gtransform.forward();
        let inv_transform = gtransform.compute_matrix().inverse();

        transform.translation = (inv_transform * global_pos.extend(1.0)).xyz();
    }
}
