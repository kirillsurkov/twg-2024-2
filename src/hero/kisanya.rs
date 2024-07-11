use std::time::Duration;

use bevy::{gltf::Gltf, prelude::*};

use crate::component::{
    complex_anim_player::{
        self, Animations, ComplexAnimPart, ComplexAnimPlayer, Showoff, SHOWOFF_IMMEDIATE,
        SHOWOFF_LAZY,
    },
    land,
    model::Model,
    wheel,
};

use super::LocalSchedule;

#[derive(Component)]
pub struct Kisanya;

#[derive(Component)]
pub struct Ready;

#[derive(Component)]
pub struct ModelReady;

impl Plugin for Kisanya {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (on_add, filter_animations, on_wheel, on_land),
        );
    }
}

fn on_add(
    mut commands: Commands,
    model: Option<Res<Model<Kisanya>>>,
    asset_server: Res<AssetServer>,
    assets_gltf: Res<Assets<Gltf>>,
    query: Query<Entity, (With<Kisanya>, Without<Ready>)>,
    query_model: Query<Entity, (With<Kisanya>, Without<ModelReady>)>,
    query_animation: Query<Entity, (With<Kisanya>, With<ModelReady>)>,
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
            commands.insert_resource(Model::<Kisanya>::new(
                asset_server.load("embedded://kisanya.glb"),
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
                            .with_showoff(Showoff::new(vec![
                                ComplexAnimPart {
                                    name: "nipples_ready_track".to_string(),
                                    repeat: 1,
                                    speed: 1.0,
                                    wait: Duration::from_millis(0),
                                },
                                ComplexAnimPart {
                                    name: "nipples_rug_track".to_string(),
                                    repeat: 3,
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

fn filter_animations(
    mut query: Query<&Animations, With<Kisanya>>,
    mut named: Query<(&Name, &mut Visibility)>,
) {
}

fn on_wheel(mut query: Query<(&mut ComplexAnimPlayer, &wheel::HeroState), With<Kisanya>>) {
    for (mut anim_player, state) in query.iter_mut() {
        if state.active {
            anim_player.play(state.changed, SHOWOFF_LAZY);
        } else {
            anim_player.play(state.changed, complex_anim_player::State::Idle);
        }
    }
}

fn on_land(mut query: Query<&mut ComplexAnimPlayer, (With<land::HeroState>, With<Kisanya>)>) {
    for mut anim_player in query.iter_mut() {
        anim_player.play(false, SHOWOFF_IMMEDIATE);
    }
}
