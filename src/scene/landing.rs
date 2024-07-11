use std::error::Error;

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

use crate::{
    battle::{player::Player, Battle},
    battle_bridge::{BattleResource, HeroesResource},
    component::land::Land,
    hero::HeroesRoot,
};

use super::{GameState, LocalSchedule, Root};

#[derive(Resource)]
struct State {
    timer: f32,
}

pub struct Landing;

impl Plugin for Landing {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (init.map(drop), update.run_if(resource_exists::<State>))
                .run_if(in_state(GameState::Landing)),
        );
    }
}

#[derive(Resource)]
pub struct HeroSelected {
    pub id: String,
}

fn init(
    mut commands: Commands,
    selected: Res<HeroSelected>,
    root: Query<Entity, Added<Root>>,
) -> Result<(), Box<dyn Error>> {
    let root = root.get_single()?;
    println!("LANDING INIT FOR {}", selected.id);
    commands.insert_resource(State { timer: 0.0 });
    commands.entity(root).with_children(|p| {
        p.spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(-2.0, 10.0, 10.0))
                    .looking_at(Vec3::new(0.0, 0.0, -5.0), Vec3::Y),
                ..default()
            },
            BloomSettings::default(),
        ));

        p.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::rgb(0.98, 0.95, 0.82),
                shadows_enabled: true,
                illuminance: 1000.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .looking_at(Vec3::new(0.15, -0.15, -0.25), Vec3::Y),
            ..Default::default()
        });

        p.spawn((Land::new(), HeroesRoot));
    });
    Ok(())
}

fn update(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut state: ResMut<State>,
    heroes: Res<HeroesResource>,
    time: Res<Time>,
    land: Query<&Land>,
) {
    let land = land.single();

    if land.ready() {
        state.timer += time.delta_seconds();
        if state.timer >= 3.0 {
            commands.insert_resource(BattleResource(Battle::new(
                heroes.iter().map(|(h, _)| Player::new(h.clone())).collect(),
            )));
            next_state.set(GameState::FightHome);
        }
    }
}
