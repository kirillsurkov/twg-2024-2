use bevy::prelude::*;

use crate::component::game_timer::GameTimer;

use super::LocalSchedule;

pub struct GameTimerPlugin;

impl Plugin for GameTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, (init_root, update_timer));
    }
}

#[derive(Component)]
pub struct GameTimerRoot;

fn init_root(mut commands: Commands, query: Query<Entity, Added<GameTimerRoot>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    width: Val::Px(100.0),
                    height: Val::Percent(100.0),
                    margin: UiRect::left(Val::Auto),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: Color::BLACK.with_a(0.5).into(),
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((
                    GameTimerText,
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font_size: 50.0,
                            ..Default::default()
                        },
                    ),
                ));
            });
    }
}

#[derive(Component)]
struct GameTimerText;

fn update_timer(mut query: Query<&mut Text, With<GameTimerText>>, state: Res<GameTimer>) {
    for mut text in query.iter_mut() {
        let section = &mut text.sections[0];
        if state.red {
            section.style.color = Color::RED;
        } else {
            section.style.color = Color::WHITE;
        }
        section.value = format!("{:.0}", (state.max - state.value).ceil() - 1.0);
    }
}
