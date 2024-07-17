use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::battle::{card::CardBranch, hero::Hero, Battle, RoundCapture};

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
        self.0.iter().find(|c| match c {
            RoundCapture::Fight {
                player1, player2, ..
            } => *player1 == id || *player2 == id,
            RoundCapture::Skip(player) => *player == id,
        })
    }
}

pub fn branch_to_color(branch: &CardBranch) -> Color {
    match branch {
        CardBranch::Attack => Color::CRIMSON,
        CardBranch::Regen => Color::YELLOW_GREEN,
        CardBranch::Hp => Color::YELLOW,
        CardBranch::Mana => Color::CYAN,
        CardBranch::Crit => Color::ORANGE,
        CardBranch::Evasion => Color::PURPLE,
    }
}
