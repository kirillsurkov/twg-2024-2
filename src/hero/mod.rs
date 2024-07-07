use std::marker::PhantomData;

use bevy::{ecs::schedule::ScheduleLabel, gltf::Gltf, prelude::*};
use dimas::Dimas;
use dtyan::DTyan;
use duck::Duck;
use nulch::Nulch;
use rasp::Rasp;

#[derive(Resource)]
pub struct Model<T> {
    handle: Handle<Gltf>,
    _pd: PhantomData<T>,
}

impl<T> Model<T> {
    pub fn new(gltf: Handle<Gltf>) -> Self {
        Self {
            handle: gltf,
            _pd: PhantomData::default(),
        }
    }
}

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct LocalSchedule;

#[derive(Component)]
pub struct HeroesRoot;

pub struct HeroesPlugin;

impl Plugin for HeroesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Nulch, Rasp, DTyan, Dimas, Duck));
        app.add_systems(LocalSchedule, init_heroes);
    }
}

fn init_heroes(mut commands: Commands, query: Query<Entity, Added<HeroesRoot>>) {
    for root in query.iter() {
        println!("HEROES INIT");
        commands.entity(root).with_children(|p| {
            p.spawn((Nulch, Name::new("nulch")));
            p.spawn((Rasp, Name::new("rasp")));
            p.spawn((DTyan, Name::new("dtyan")));
            p.spawn((Dimas, Name::new("dimas")));
            p.spawn((Duck, Name::new("duck")));
        });
    }
}

pub mod dimas;
pub mod dtyan;
pub mod duck;
pub mod nulch;
pub mod rasp;
