use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct LocalSchedule;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Splash,
    SelectHero,
    Landing,
    FightHome,
    FightArena,
}

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::default());
        app.add_systems(LocalSchedule, cleanup.run_if(state_changed::<GameState>));
        app.add_systems(
            LocalSchedule,
            (
                splash::update.run_if(in_state(GameState::Splash)),
                select_hero::update.run_if(in_state(GameState::SelectHero)),
                landing::update.run_if(in_state(GameState::Landing)),
                fight_home::update.run_if(in_state(GameState::FightHome)),
                fight_arena::update.run_if(in_state(GameState::FightArena)),
            )
                .before(cleanup)
                .run_if(any_with_component::<Root>)
                .run_if(not(state_changed::<GameState>)),
        );
    }
}

#[derive(Component)]
pub struct Root;

#[derive(Component)]
pub struct InitScene;

fn cleanup(mut commands: Commands, query: Query<Entity, With<Root>>) {
    println!("CLEANUP");
    if let Ok(root) = query.get_single() {
        commands.entity(root).despawn_recursive();
    }
    commands.spawn((
        Root,
        InitScene,
        TransformBundle::default(),
        VisibilityBundle::default(),
    ));
}

pub mod fight_arena;
pub mod fight_home;
pub mod landing;
pub mod select_hero;
pub mod splash;
