use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use dimas::Dimas;
use dtyan::DTyan;
use duck::Duck;
use kisanya::Kisanya;
use nulch::Nulch;
use rasp::Rasp;

use crate::battle::hero::{dimas, dtyan, duck, kisanya, nulch, rasp, Hero};

#[derive(Component, Deref)]
pub struct HeroComponent(pub Hero);

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct LocalSchedule;

#[derive(Component)]
pub struct HeroesRoot;

pub struct HeroesPlugin;

impl Plugin for HeroesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Nulch, Rasp, DTyan, Dimas, Duck, Kisanya));
        app.add_systems(LocalSchedule, init_heroes);
    }
}

fn init_heroes(mut commands: Commands, query: Query<Entity, Added<HeroesRoot>>) {
    for root in query.iter() {
        println!("HEROES INIT");
        commands.entity(root).with_children(|p| {
            p.spawn((Nulch, HeroComponent(nulch())));
            p.spawn((Rasp, HeroComponent(rasp())));
            p.spawn((DTyan, HeroComponent(dtyan())));
            p.spawn((Dimas, HeroComponent(dimas())));
            p.spawn((Duck, HeroComponent(duck())));
            p.spawn((Kisanya, HeroComponent(kisanya())));
        });
    }
}

pub mod dimas;
pub mod dtyan;
pub mod duck;
pub mod kisanya;
pub mod nulch;
pub mod rasp;
