use std::{f32::consts::FRAC_PI_6, time::Duration};

use bevy::{gltf::Gltf, prelude::*};

use crate::{
    component::{
        complex_anim_player::{
            self, Animations, ComplexAnimPart, ComplexAnimPlayer, Showoff, SHOWOFF_IMMEDIATE,
            SHOWOFF_LAZY,
        },
        land,
        model::Model,
        wheel,
    },
    scene::avatars::{self, AvatarLocation},
};

use super::LocalSchedule;

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
            (on_add, filter_animations, on_avatar, on_wheel, on_land),
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
                            .with_showoff(Showoff::new(vec![ComplexAnimPart {
                                name: "drink_track".to_string(),
                                repeat: 1,
                                speed: 1.0,
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
    mut query: Query<&Animations, With<Nulch>>,
    mut named: Query<(&Name, &mut Visibility)>,
) {
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

fn on_wheel(mut query: Query<(&mut ComplexAnimPlayer, &wheel::HeroState), With<Nulch>>) {
    for (mut anim_player, state) in query.iter_mut() {
        if state.active {
            anim_player.play(state.changed, SHOWOFF_LAZY);
        } else {
            anim_player.play(state.changed, complex_anim_player::State::Idle);
        }
    }
}

fn on_land(mut query: Query<&mut ComplexAnimPlayer, (With<land::HeroState>, With<Nulch>)>) {
    for mut anim_player in query.iter_mut() {
        anim_player.play(false, SHOWOFF_IMMEDIATE);
    }
}
