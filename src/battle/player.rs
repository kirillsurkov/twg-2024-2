use super::{effect::HasEffect, hero::Hero};

pub struct Player {
    pub hero: Hero,
    pub money: u32,
    pub attack: u32,
    pub hp: u32,
    pub cards: Vec<Box<dyn HasEffect + Send + Sync>>,
}

impl Player {
    pub fn new(hero: Hero) -> Self {
        Self {
            hero,
            money: 300,
            attack: 3,
            hp: 50,
            cards: vec![],
        }
    }
}
