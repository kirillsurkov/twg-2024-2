use std::error::Error;

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use dimas::Dimas;
use dtyan::DTyan;
use duck::Duck;
use kisanya::Kisanya;
use nulch::Nulch;
use rasp::Rasp;

use crate::{
    battle::hero::{dimas, dtyan, duck, kisanya, nulch, rasp},
    battle_bridge::HeroesResource,
};

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct LocalSchedule;

#[derive(Component)]
pub struct HeroesRoot;

#[derive(Component, Deref)]
pub struct HeroId(pub String);

pub struct HeroesPlugin;

impl Plugin for HeroesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Nulch, Rasp, DTyan, Dimas, Duck, Kisanya));
        app.add_systems(LocalSchedule, init_heroes);
        app.insert_resource(HeroesResource(vec![
            (nulch(), Box::new(|cmd| cmd.spawn(Nulch))),
            (rasp(), Box::new(|cmd| cmd.spawn(Rasp))),
            (dtyan(), Box::new(|cmd| cmd.spawn(DTyan))),
            (dimas(), Box::new(|cmd| cmd.spawn(Dimas))),
            (duck(), Box::new(|cmd| cmd.spawn(Duck))),
            (kisanya(), Box::new(|cmd| cmd.spawn(Kisanya))),
        ]));
    }
}

fn init_heroes(
    mut commands: Commands,
    root: Query<Entity, Added<HeroesRoot>>,
    heroes: Res<HeroesResource>,
) {
    for root in root.iter() {
        println!("HEROES INIT");
        commands.entity(root).with_children(|p| {
            heroes.iter().for_each(|(hero, spawn)| {
                spawn(p).insert(HeroId(hero.id.to_string()));
            })
        });
    }
}

pub mod dimas;
pub mod dtyan;
pub mod duck;
pub mod kisanya;
pub mod nulch;
pub mod rasp;
