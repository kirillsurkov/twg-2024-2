use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::battle::{hero::Hero, player::Player, Battle};

#[derive(Component, Deref)]
pub struct PlayerComponent(pub Player);

#[derive(Resource, Deref)]
pub struct HeroesResource(
    pub  Vec<(
        Hero,
        Box<dyn for<'a> Fn(&'a mut ChildBuilder) -> EntityCommands<'a> + Send + Sync>,
    )>,
);

#[derive(Resource, Deref)]
pub struct BattleResource(pub Battle);
