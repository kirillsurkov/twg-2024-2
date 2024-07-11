use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::battle::{hero::Hero, Battle, RoundCapture};

#[derive(Resource, Deref)]
pub struct HeroesResource(
    pub  Vec<(
        Hero,
        Box<dyn for<'a> Fn(&'a mut ChildBuilder) -> EntityCommands<'a> + Send + Sync>,
    )>,
);

#[derive(Resource, Deref, DerefMut)]
pub struct BattleResource(pub Battle);

#[derive(Resource)]
pub struct RoundCaptureResource(pub Vec<RoundCapture>);

impl RoundCaptureResource {
    pub fn by_player(&self, id: &str) -> Option<&RoundCapture> {
        self.0.iter().find(|c| c.player1 == id || c.player2 == id)
    }
}
