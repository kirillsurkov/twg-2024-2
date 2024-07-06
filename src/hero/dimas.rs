use std::time::Duration;

use bevy::{gltf::Gltf, prelude::*};

use crate::{
    complex_anim_player::{self, Animations, ComplexAnimPart, ComplexAnimPlayer, Showoff},
    wheel,
};

use super::{Hero, Model};

#[derive(Component)]
pub struct Dimas;

#[derive(Component)]
pub struct Ready;

#[derive(Component)]
pub struct ModelReady;

impl Hero for Dimas {
    fn register(app: &mut App) {
        app.add_systems(Update, (on_add, on_wheel));
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
            commands.insert_resource(Model::<Dimas>::new(asset_server.load("dimas.glb")));
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

fn on_wheel(mut query: Query<(&mut ComplexAnimPlayer, &wheel::State), With<Dimas>>) {
    for (mut anim_player, state) in query.iter_mut() {
        if state.active {
            anim_player.play(complex_anim_player::State::Showoff);
        } else {
            anim_player.play(complex_anim_player::State::Idle);
        }
    }
}
