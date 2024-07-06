use std::{marker::PhantomData, time::Duration};

use bevy::{animation::RepeatAnimation, gltf::Gltf, prelude::*};

use crate::{
    complex_anim_player::{self, Animations, ComplexAnimPart, ComplexAnimPlayer, Showoff},
    wheel,
};

use super::Hero;

#[derive(Resource)]
pub struct Model<T> {
    handle: Handle<Gltf>,
    _pd: PhantomData<T>,
}

impl<T> Model<T> {
    pub fn new(gltf: Handle<Gltf>) -> Self {
        Self {
            handle: gltf,
            _pd: PhantomData::default(),
        }
    }
}

#[derive(Component)]
pub struct Rasp;

#[derive(Component)]
pub struct Ready;

#[derive(Component)]
pub struct ModelReady;

impl Hero for Rasp {
    fn register(app: &mut App) {
        app.add_systems(Update, (on_add, on_wheel));
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
            commands.insert_resource(Model::<Rasp>::new(asset_server.load("rasp.glb")));
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
                            .with_showoff(Showoff::new(vec![
                                ComplexAnimPart {
                                    name: "idle_track".to_string(),
                                    repeat: RepeatAnimation::Count(2),
                                    speed: 1.0,
                                    wait: Duration::from_millis(0),
                                },
                                ComplexAnimPart {
                                    name: "start_shoot_track".to_string(),
                                    repeat: RepeatAnimation::Count(1),
                                    speed: 1.0,
                                    wait: Duration::from_millis(1000),
                                },
                                ComplexAnimPart {
                                    name: "shoot_track".to_string(),
                                    repeat: RepeatAnimation::Count(3),
                                    speed: 3.0,
                                    wait: Duration::from_millis(500),
                                },
                            ])),
                        Animations {
                            by_name: gltf.named_animations.clone(),
                        },
                    ));
                }
            }
        }
    }
}

fn on_wheel(mut query: Query<(&mut ComplexAnimPlayer, &wheel::State), With<Rasp>>) {
    for (mut anim_player, state) in query.iter_mut() {
        if state.active {
            anim_player.play(complex_anim_player::State::Showoff);
        } else {
            anim_player.play(complex_anim_player::State::Idle);
        }
    }
}
