use std::error::Error;

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

use crate::{
    battle_bridge::HeroesResource, component::wheel::Wheel, hero::HeroesRoot,
    scene::landing::HeroSelected,
};

use super::{landing::HeroWatch, GameState, LocalSchedule, Root, UiRoot};

#[derive(Resource, Default)]
struct State {
    timer: f32,
}

pub struct SelectHero;

impl Plugin for SelectHero {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (init.map(drop), update.run_if(resource_exists::<State>))
                .run_if(in_state(GameState::SelectHero)),
        );
    }
}

#[derive(Component)]
struct DescNode;

#[derive(Component)]
struct NameNode;

#[derive(Component)]
struct StatsNode;

fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    root: Query<Entity, Added<Root>>,
) -> Result<(), Box<dyn Error>> {
    let root = root.get_single()?;
    commands.insert_resource(State::default());
    commands.entity(root).with_children(|p| {
        p.spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(-0.5, 3.0, 6.0))
                    .looking_at(Vec3::new(0.0, 1.5, 0.0), Vec3::Y),
                ..Default::default()
            },
            BloomSettings::default(),
        ));

        p.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::rgb(0.98, 0.95, 0.82),
                shadows_enabled: true,
                illuminance: 1000.0,
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .looking_at(Vec3::new(0.15, -0.15, -0.25), Vec3::Y),
            ..Default::default()
        });

        p.spawn((Wheel::new(10.0), HeroesRoot));

        // p.spawn(AudioBundle {
        //     source: asset_server.load("embedded://rockafeller_skank.ogg"),
        //     ..Default::default()
        // });
    });

    let font: Handle<Font> = asset_server.load("embedded://comic.ttf");

    commands
        .spawn((
            UiRoot,
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    display: Display::Flex,
                    align_items: AlignItems::FlexStart,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|p| {
            p.spawn(NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Percent(5.0)),
                    padding: UiRect::all(Val::Px(25.0)),
                    width: Val::Percent(20.0),
                    align_self: AlignSelf::Center,
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((
                    DescNode,
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font: font.clone(),
                            font_size: 25.0,
                            ..Default::default()
                        },
                    ),
                ));
            });
            p.spawn(NodeBundle {
                style: Style {
                    margin: UiRect::axes(Val::Percent(10.0), Val::Px(50.0)),
                    padding: UiRect::all(Val::Px(25.0)),
                    width: Val::Percent(30.0),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((
                    NameNode,
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font: font.clone(),
                            font_size: 50.0,
                            ..Default::default()
                        },
                    ),
                ));
            });
            p.spawn(NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Percent(5.0)),
                    padding: UiRect::all(Val::Px(25.0)),
                    width: Val::Percent(20.0),
                    align_self: AlignSelf::Center,
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((
                    StatsNode,
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font: font.clone(),
                            font_size: 25.0,
                            ..Default::default()
                        },
                    ),
                ));
            });
        });
    Ok(())
}

fn update(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut state: ResMut<State>,
    mut desc_node: Query<&mut Text, (With<DescNode>, Without<NameNode>, Without<StatsNode>)>,
    mut name_node: Query<&mut Text, (Without<DescNode>, With<NameNode>, Without<StatsNode>)>,
    mut stats_node: Query<&mut Text, (Without<DescNode>, Without<NameNode>, With<StatsNode>)>,
    time: Res<Time>,
    heroes: Res<HeroesResource>,
    wheel: Query<&Wheel>,
) {
    let wheel = wheel.single();

    let (ref selected_hero, _) = heroes[wheel.current()];

    if wheel.changed() {
        for mut text in desc_node.iter_mut() {
            text.sections[0].value = selected_hero.desc.to_string();
        }

        for mut text in name_node.iter_mut() {
            text.sections[0].value = selected_hero.name.to_string();
        }

        for mut text in stats_node.iter_mut() {
            text.sections[0].value = format!("HP: {hp}\nMana regen: {mana}\nAttack: {attack}\nAttack speed: {aps}\nCrit: {crit:.0}%\nEvasion: {evasion:.0}%",
            hp=selected_hero.hp,
            mana=selected_hero.mana_regen,
            attack=selected_hero.attack,
            aps=selected_hero.attack_speed,
            crit=selected_hero.crit*100.0,
            evasion=selected_hero.evasion*100.0
        );
        }
    }

    if wheel.selected() {
        commands.insert_resource(HeroSelected {
            id: selected_hero.id.to_string(),
        });
        commands.insert_resource(HeroWatch {
            id: selected_hero.id.to_string(),
        });
        next_state.set(GameState::Landing);
    }

    state.timer += time.delta_seconds();
    if state.timer >= 1.0 {
        // next_state.set(GameState::Landing);
    }
}
