use bevy::prelude::*;

#[derive(Component)]
pub struct Avatar;

#[derive(Component)]
pub struct Gameplay;

pub trait Hero {
    fn register(app: &mut App);
}

#[derive(Default)]
pub struct HeroPlugin {
    heroes: Vec<Box<dyn Fn(&mut App) + Send + Sync>>,
}

impl HeroPlugin {
    pub fn with_hero<T: Hero>(mut self) -> Self {
        self.heroes.push(Box::new(|app| {
            T::register(app);
        }));
        self
    }
}

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        for hero in &self.heroes {
            hero(app);
        }
    }
}

pub mod dtyan;
pub mod dimas;
pub mod duck;
pub mod nulch;
pub mod rasp;
