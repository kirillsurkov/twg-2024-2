use super::{effect::HasEffect, hero::Hero};

pub struct Player {
    pub hero: Hero,
    pub cards: Vec<Box<dyn HasEffect + Send + Sync>>,
}

impl Player {
    pub fn new(hero: Hero) -> Self {
        Self {
            hero,
            cards: vec![],
        }
    }
}
