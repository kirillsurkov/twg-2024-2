use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use dimas::Dimas;
use dtyan::DTyan;
use duck::Duck;
use kisanya::Kisanya;
use nulch::Nulch;
use rasp::Rasp;

use crate::{
    battle::{
        fight::Owner,
        hero::{dimas, dtyan, duck, kisanya, nulch, rasp},
        modifier::Modifier,
    },
    battle_bridge::HeroesResource,
    component::{
        arena,
        complex_anim_player::{self, ComplexAnimPlayer, SHOWOFF_IMMEDIATE, SHOWOFF_LAZY},
        fight_state::FightState,
        land,
        projectile::Projectile,
        wheel,
    },
    scene::Root,
};

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct LocalSchedule;

#[derive(Component)]
pub struct HeroesRoot;

#[derive(Component, Deref, Clone)]
pub struct HeroId(pub String);

pub struct HeroesPlugin;

impl Plugin for HeroesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Nulch, Rasp, DTyan, Dimas, Duck, Kisanya));
        app.add_systems(
            LocalSchedule,
            (
                init_heroes,
                on_wheel,
                on_land,
                (on_arena, on_arena_projectiles).run_if(resource_exists::<FightState>),
            ),
        );
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

fn on_wheel(mut query: Query<(&mut ComplexAnimPlayer, &wheel::HeroState), With<HeroId>>) {
    for (mut anim_player, state) in query.iter_mut() {
        if state.active {
            anim_player.play(state.changed, SHOWOFF_LAZY);
        } else {
            anim_player.play(state.changed, complex_anim_player::State::Idle);
        }
    }
}

fn on_land(mut query: Query<&mut ComplexAnimPlayer, (With<land::HeroState>, With<HeroId>)>) {
    for mut anim_player in query.iter_mut() {
        anim_player.play(false, SHOWOFF_IMMEDIATE);
    }
}

fn on_arena(
    mut query: Query<(&mut ComplexAnimPlayer, &HeroId), With<arena::HeroState>>,
    fight: Res<FightState>,
) {
    for (mut anim_player, id) in query.iter_mut() {
        let owner = if fight.fighter1.hero.id == id.0 {
            Owner::Fighter1
        } else if fight.fighter2.hero.id == id.0 {
            Owner::Fighter2
        } else {
            continue;
        };

        if let Some(winner) = fight.winner {
            if winner == owner {
                anim_player.play(false, complex_anim_player::State::Win);
            } else {
                anim_player.play(false, complex_anim_player::State::Lose);
            }
        } else {
            anim_player.play(
                false,
                complex_anim_player::State::Attack(match owner {
                    Owner::Fighter1 => fight.fighter1.attack_speed,
                    Owner::Fighter2 => fight.fighter2.attack_speed,
                }),
            );
        }
    }
}

fn on_arena_projectiles(
    mut commands: Commands,
    mut query: Query<&HeroId, With<arena::HeroState>>,
    fight: Res<FightState>,
    heroes: Query<(Entity, &HeroId), With<arena::HeroState>>,
    root: Query<Entity, With<Root>>,
) {
    let Ok(root) = root.get_single() else {
        return;
    };

    for id in query.iter_mut() {
        if fight.winner.is_some() {
            continue;
        }

        let owner = if fight.fighter1.hero.id == id.0 {
            Owner::Fighter1
        } else if fight.fighter2.hero.id == id.0 {
            Owner::Fighter2
        } else {
            continue;
        };

        let (myself_id, enemy_id) = {
            (
                id.0.as_str(),
                match owner {
                    Owner::Fighter1 => fight.fighter2.hero.id,
                    Owner::Fighter2 => fight.fighter1.hero.id,
                },
            )
        };

        let myself = heroes.iter().find(|(_, id)| id.0 == myself_id).unwrap().0;
        let enemy = heroes.iter().find(|(_, id)| id.0 == enemy_id).unwrap().0;

        for (o, m) in &fight.modifiers {
            if *o != owner {
                continue;
            }
            match m.modifier {
                Modifier::NormalAttack => {
                    commands.entity(root).with_children(|p| {
                        p.spawn((Projectile::new(myself, enemy, 0.5), id.clone()));
                    });
                }
                _ => {}
            }
        }
    }
}

pub mod dimas;
pub mod dtyan;
pub mod duck;
pub mod kisanya;
pub mod nulch;
pub mod rasp;
