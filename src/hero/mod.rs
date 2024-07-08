use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use dimas::Dimas;
use dtyan::DTyan;
use duck::Duck;
use kisanya::Kisanya;
use nulch::Nulch;
use rasp::Rasp;

#[derive(Component, Clone)]
pub struct Hero {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub hp: f32,
    pub mana_regen: f32,
    pub attack: f32,
    pub attack_speed: f32,
    pub crit: f32,
    pub evasion: f32,
}

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
            p.spawn((
                Nulch,
                Hero {
                    id: "nulch".to_string(),
                    name: "Нульч".to_string(),
                    desc: "Организатор конкурса".to_string(),
                    hp: 1100.0,
                    mana_regen: 10.0,
                    attack: 25.0,
                    attack_speed: 0.95,
                    crit: 0.16,
                    evasion: 0.15,
                },
            ));
            p.spawn((
                Rasp,
                Hero {
                    id: "rasp".to_string(),
                    name: "Расп".to_string(),
                    desc: "Это твоя мамка, анон".to_string(),
                    hp: 1200.0,
                    mana_regen: 10.0,
                    attack: 22.0,
                    attack_speed: 1.04,
                    crit: 0.11,
                    evasion: 0.18,
                },
            ));
            p.spawn((
                DTyan,
                Hero {
                    id: "dtyan".to_string(),
                    name: "Деревотян".to_string(),
                    desc: "Вырасти дерево, если сможешь!".to_string(),
                    hp: 1400.0,
                    mana_regen: 10.0,
                    attack: 34.0,
                    attack_speed: 0.75,
                    crit: 0.20,
                    evasion: 0.08,
                },
            ));
            p.spawn((
                Dimas,
                Hero {
                    id: "dimas".to_string(),
                    name: "Димасик".to_string(),
                    desc: "Организатор предыдущего конкурса".to_string(),
                    hp: 1350.0,
                    mana_regen: 11.0,
                    attack: 35.0,
                    attack_speed: 0.71,
                    crit: 0.17,
                    evasion: 0.09,
                },
            ));
            p.spawn((
                Duck,
                Hero {
                    id: "duck".to_string(),
                    name: "Утка".to_string(),
                    desc: "Умеет делиться пополам".to_string(),
                    hp: 1200.0,
                    mana_regen: 10.0,
                    attack: 24.0,
                    attack_speed: 0.95,
                    crit: 0.15,
                    evasion: 0.14,
                },
            ));
            p.spawn((
                Kisanya,
                Hero {
                    id: "kisanya".to_string(),
                    name: "Кисаня".to_string(),
                    desc: "Обязательно пройдёт твою игру на стриме".to_string(),
                    hp: 1100.0,
                    mana_regen: 9.0,
                    attack: 22.0,
                    attack_speed: 1.11,
                    crit: 0.15,
                    evasion: 0.12,
                },
            ));
        });
    }
}

pub mod dimas;
pub mod dtyan;
pub mod duck;
pub mod kisanya;
pub mod nulch;
pub mod rasp;
