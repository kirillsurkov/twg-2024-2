use std::time::Duration;

use bevy::{gltf::Gltf, prelude::*};

use crate::{
    complex_anim_player::{self, Animations, ComplexAnimPart, ComplexAnimPlayer, Showoff},
    wheel,
};

use super::{Hero, Model};

#[derive(Component)]
pub struct Nulch;

#[derive(Component)]
pub struct Ready;

#[derive(Component)]
pub struct ModelReady;

impl Hero for Nulch {
    fn register(app: &mut App) {
        app.add_systems(Update, (on_add, on_wheel));
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
            commands.insert_resource(Model::<Nulch>::new(asset_server.load("nulch.glb")));
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

fn on_wheel(mut query: Query<(&mut ComplexAnimPlayer, &wheel::State), With<Nulch>>) {
    for (mut anim_player, state) in query.iter_mut() {
        if state.active {
            anim_player.play(complex_anim_player::State::Showoff);
        } else {
            anim_player.play(complex_anim_player::State::Idle);
        }
    }
}
