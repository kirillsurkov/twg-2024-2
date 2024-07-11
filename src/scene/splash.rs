use std::error::Error;

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

use crate::scene::UiRoot;

use super::{GameState, LocalSchedule, Root};

#[derive(Resource)]
struct State {
    timer: f32,
}

pub struct Splash;

impl Plugin for Splash {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (init.map(drop), update.run_if(resource_exists::<State>))
                .run_if(in_state(GameState::Splash)),
        );
    }
}

fn init(
    mut commands: Commands,
    root: Query<Entity, Added<Root>>,
    asset_server: Res<AssetServer>,
) -> Result<(), Box<dyn Error>> {
    let root = root.get_single()?;
    println!("SPLASH INIT");
    commands.insert_resource(State { timer: 0.0 });

    commands.entity(root).with_children(|p| {
        p.spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 5.0, 5.0))
                    .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
                ..default()
            },
            BloomSettings::default(),
        ));
    });

    let font: Handle<Font> = asset_server.load("embedded://comic.ttf");

    let text_card = TextStyle {
        font: font.clone(),
        font_size: 50.0,
        ..Default::default()
    };

    commands
        .spawn((
            UiRoot,
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                // background_color: Color::BLUE.into(),
                ..Default::default()
            },
        ))
        .with_children(|p| {
            p.spawn((TextBundle::from_section(
                "TWG: UNITED (ВСЕ В СБОРЕ)",
                text_card.clone(),
            ),));
        });
    Ok(())
}

fn update(mut next_state: ResMut<NextState<GameState>>, mut state: ResMut<State>, time: Res<Time>) {
    state.timer += time.delta_seconds();
    if state.timer >= 2.0 {
        next_state.set(GameState::SelectHero);
    }
}
