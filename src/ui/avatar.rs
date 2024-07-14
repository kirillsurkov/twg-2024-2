use bevy::prelude::*;

use crate::{
    battle_bridge::{BattleResource, RoundCaptureResource},
    hero::HeroId,
    scene::{
        avatars::{Avatar, AvatarsResource},
        landing::{HeroSelected, HeroWatch},
    },
};

use super::{LocalSchedule, UiAssets, DCOLOR};

pub struct AvatarPlugin;

const HEIGHT: f32 = 50.0;

impl Plugin for AvatarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (init_avatar_root,).run_if(resource_exists::<BattleResource>),
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
                    width: Val::Px(200.0),
                    ..Default::default()
                },
                background_color: DCOLOR,
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn(ImageBundle {
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
