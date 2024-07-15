use std::{
    f32::consts::{FRAC_PI_2, FRAC_PI_3},
    time::Duration,
};

use bevy::{gltf::Gltf, prelude::*};

use crate::{
    component::{
        arena,
        complex_anim_player::{self, Animations, ComplexAnimPart, ComplexAnimPlayer, Showoff},
        fight_state::FightState,
        model::Model,
    },
    scene::avatars::{self, AvatarLocation},
};

use super::LocalSchedule;

#[derive(Component)]
pub struct Dimas;

#[derive(Component)]
struct Swiborg;

#[derive(Component)]
pub struct Ready;

#[derive(Component)]
pub struct ModelReady;

impl Plugin for Dimas {
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
    model: Option<Res<Model<Dimas>>>,
    swiborg: Option<Res<Model<Swiborg>>>,
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

    if swiborg.is_none() {
        commands.insert_resource(Model::<Swiborg>::new(
            asset_server.load("embedded://swiborg.glb"),
        ));
    }

    for entity in query_model.iter() {
        commands
            .entity(entity)
            .insert(ModelReady)
            .with_children(|p| {
                p.spawn(SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    transform: Transform::from_scale(Vec3::splat(1.0)),
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

fn on_arena(mut query: Query<&mut Transform, (With<arena::HeroState>, With<Dimas>)>) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_rotation_y(-FRAC_PI_2);
    }
}
