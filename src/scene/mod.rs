use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use fight_arena::FightArena;
use fight_home::FightHome;
use landing::{HeroSelected, Landing};
use select_hero::SelectHero;
use splash::Splash;

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

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::default());
        app.add_plugins((Splash, SelectHero, Landing, FightHome, FightArena));
        app.add_systems(Update, cleanup.run_if(state_changed::<GameState>));

        app.insert_state(GameState::Landing);
        app.insert_resource(HeroSelected {
            id: "dimas".to_string(),
        });
    }
}

#[derive(Component)]
pub struct Root;

#[derive(Component)]
pub struct UiRoot;

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<Root>>,
    query_ui: Query<Entity, With<UiRoot>>,
) {
    println!("CLEANUP");
    if let Ok(root) = query.get_single() {
        commands.entity(root).despawn_recursive();
    }
    if let Ok(ui_root) = query_ui.get_single() {
        commands.entity(ui_root).despawn_recursive();
    }
    commands.spawn((
        Root,
        TransformBundle::default(),
        VisibilityBundle::default(),
    ));
}

pub mod fight_arena;
pub mod fight_home;
pub mod landing;
pub mod select_hero;
pub mod splash;
