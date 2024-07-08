use std::time::Duration;

use bevy::{gltf::Gltf, prelude::*};

use crate::component::{
    complex_anim_player::{self, Animations, ComplexAnimPart, ComplexAnimPlayer, Showoff}, model::Model, wheel
};

use super::LocalSchedule;

#[derive(Component)]
pub struct DTyan;

#[derive(Component)]
pub struct Ready;

#[derive(Component)]
pub struct ModelReady;

impl Plugin for DTyan {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, (on_add, on_wheel));
    }
}

fn on_add(
    mut commands: Commands,
    model: Option<Res<Model<DTyan>>>,
    asset_server: Res<AssetServer>,
    assets_gltf: Res<Assets<Gltf>>,
    query: Query<Entity, (With<DTyan>, Without<Ready>)>,
    query_model: Query<Entity, (With<DTyan>, Without<ModelReady>)>,
    query_animation: Query<Entity, (With<DTyan>, With<ModelReady>)>,
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
            commands.insert_resource(Model::<DTyan>::new(asset_server.load("derevotyan.glb")));
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
                                    name: "hand_to_glasses_track".to_string(),
                                    repeat: 1,
                                    speed: 1.0,
                                    wait: Duration::from_millis(500),
                                },
                                ComplexAnimPart {
                                    name: "glasses_off_track".to_string(),
                                    repeat: 1,
                                    speed: 1.0,
                                    wait: Duration::from_millis(1000),
                                },
                                ComplexAnimPart {
                                    name: "glasses_on_track".to_string(),
                                    repeat: 1,
                                    speed: 1.0,
                                    wait: Duration::from_millis(0),
                                },
                            ])),
                        Animations::new(gltf.named_animations.clone()),
                    ));
                }
            }
        }
    }
}

fn on_wheel(
    mut query: Query<(&mut ComplexAnimPlayer, &wheel::State, &Animations), With<DTyan>>,
    mut named: Query<(&Name, &mut Visibility)>,
) {
    for (mut anim_player, state, anims) in query.iter_mut() {
        let current_anim = anims.current();

        for (name, mut visibility) in named.iter_mut() {
            match name.as_str() {
                "glasses_head" => {
                    *visibility = match current_anim.as_str() {
                        "idle_track" | "hand_to_glasses_track" => Visibility::Visible,
                        _ => Visibility::Hidden,
                    }
                }
                "glasses_hand" => {
                    *visibility = match current_anim.as_str() {
                        "glasses_on_track" | "glasses_off_track" => Visibility::Visible,
                        _ => Visibility::Hidden,
                    }
                }
                _ => {}
            }
            if name.as_str() == "glasses_head" {}
        }

        if state.active {
            anim_player.play(state.changed, complex_anim_player::State::Showoff);
        } else {
            anim_player.play(state.changed, complex_anim_player::State::Idle);
        }
    }
}
