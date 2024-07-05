use std::{marker::PhantomData, time::Duration};

use bevy::{gltf::Gltf, prelude::*, utils::hashbrown::HashMap};

use crate::wheel::Active;

use super::{Hero, SelectWheel};

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

#[derive(Component, Default)]
pub struct Rasp {
    showoff: u32,
    showoff_timer: f32,
}

#[derive(Component)]
pub struct Ready;

#[derive(Component)]
pub struct ModelReady;

#[derive(Component)]
struct Animations {
    by_name: HashMap<String, Handle<AnimationClip>>,
    anim_player: Entity,
}

impl Hero for Rasp {
    fn register(app: &mut App) {
        app.add_systems(Update, (on_add, idle, showoff));
    }
}

fn on_add(
    mut commands: Commands,
    model: Option<Res<Model<Rasp>>>,
    asset_server: Res<AssetServer>,
    assets_gltf: Res<Assets<Gltf>>,
    query: Query<Entity, (With<Rasp>, Without<Ready>)>,
    query_model: Query<Entity, (With<Rasp>, Without<ModelReady>)>,
    query_animation: Query<Entity, (With<Rasp>, With<ModelReady>, Without<Animations>)>,
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
        for e in anim_players.iter() {
            for parent in children.iter_ancestors(e) {
                if parent == entity.id() {
                    println!("ADDING");
                    entity.insert(Animations {
                        by_name: gltf.named_animations.clone(),
                        anim_player: e,
                    });
                }
            }
        }
    }
}

fn idle(
    query: Query<&Animations, (With<Rasp>, Without<Active>)>,
    mut anim_players: Query<&mut AnimationPlayer>,
) {
    for anim in query.iter() {
        let mut player = anim_players.get_mut(anim.anim_player).unwrap();
        player
            .play(anim.by_name["idle_track"].clone_weak())
            .repeat();
    }
}

fn showoff(
    mut query: Query<(&mut Rasp, &Animations), With<Active>>,
    mut anim_players: Query<&mut AnimationPlayer>,
    time: Res<Time>,
) {
    for (mut rasp, anim) in query.iter_mut() {
        let mut player = anim_players.get_mut(anim.anim_player).unwrap();

        match rasp.showoff {
            0 => {
                player
                    .play_with_transition(
                        anim.by_name["idle_track"].clone_weak(),
                        Duration::from_millis(250),
                    )
                    .set_repeat(bevy::animation::RepeatAnimation::Count(2))
                    .set_speed(1.0);
                if player.is_finished() {
                    rasp.showoff = 1;
                    rasp.showoff_timer = 0.0;
                }
            }
            1 => {
                player
                    .play_with_transition(
                        anim.by_name["start_shoot_track"].clone_weak(),
                        Duration::from_millis(250),
                    )
                    .set_repeat(bevy::animation::RepeatAnimation::Never);
                if player.is_finished() {
                    rasp.showoff_timer += time.delta_seconds();
                    if rasp.showoff_timer >= 1.0 {
                        rasp.showoff = 2;
                        rasp.showoff_timer = 0.0;
                    }
                }
            }
            2 => {
                player
                    .play_with_transition(
                        anim.by_name["shoot_track"].clone_weak(),
                        Duration::from_millis(250),
                    )
                    .set_repeat(bevy::animation::RepeatAnimation::Count(3))
                    .set_speed(2.0);
                if player.is_finished() {
                    rasp.showoff_timer += time.delta_seconds();
                    if rasp.showoff_timer >= 0.5 {
                        player.replay();
                        rasp.showoff = 0;
                    }
                }
            }
            _ => {}
        }
    }
}
