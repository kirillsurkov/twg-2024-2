use bevy::prelude::*;

use crate::{
    battle_bridge::BattleResource, component::fight_state::FightState,
    scene::avatars::AvatarsResource,
};

use super::{LocalSchedule, DCOLOR};

pub struct AvatarPlugin;

const SIZE: f32 = 320.0;

impl Plugin for AvatarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (init_avatar_root, update_avatar_root.after(init_avatar_root))
                .run_if(resource_exists::<BattleResource>),
        );
    }
}

#[derive(Component)]
pub enum AvatarRoot {
    Left,
    Right,
}

fn init_avatar_root(
    mut commands: Commands,
    avatars: Res<AvatarsResource>,
    query: Query<(Entity, &AvatarRoot), Added<AvatarRoot>>,
) {
    for (entity, avatar) in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::left(match avatar {
                        AvatarRoot::Left => Val::ZERO,
                        AvatarRoot::Right => Val::Auto,
                    }),
                    width: Val::Vw(15.0),
                    height: Val::Vw(15.0),
                    padding: UiRect::all(Val::Vw(1.0)),
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn(ImageBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..Default::default()
                    },
                    image: UiImage {
                        texture: match avatar {
                            AvatarRoot::Left => avatars.left.clone(),
                            AvatarRoot::Right => avatars.right.clone(),
                        },
                        flip_x: match avatar {
                            AvatarRoot::Left => false,
                            AvatarRoot::Right => true,
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
    }
}

fn update_avatar_root(
    mut commands: Commands,
    fight: Option<Res<FightState>>,
    query: Query<(Entity, &AvatarRoot)>,
) {
    for (entity, avatar) in query.iter() {
        match avatar {
            AvatarRoot::Left => {}
            AvatarRoot::Right => {
                if fight.is_some() {
                    commands.entity(entity).insert(Visibility::Inherited);
                } else {
                    commands.entity(entity).insert(Visibility::Hidden);
                }
            }
        }
    }
}
