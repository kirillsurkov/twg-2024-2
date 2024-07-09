use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_round_ui::prelude::SuperellipseUiMaterial;

use crate::{
    component::wheel::Wheel,
    hero::{HeroComponent, HeroesRoot},
    scene::landing::HeroSelected,
};

use super::{GameState, LocalSchedule, Root, UiRoot};

#[derive(Resource, Default)]
struct State {
    timer: f32,
}

pub struct SelectHero;

impl Plugin for SelectHero {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (init, update.run_if(resource_exists::<State>)).run_if(in_state(GameState::SelectHero)),
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
    mut ui_materials: ResMut<Assets<SuperellipseUiMaterial>>,
    asset_server: Res<AssetServer>,
    query: Query<Entity, Added<Root>>,
) {
    for root in query.iter() {
        println!("SELECT HERO INIT");
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

            p.spawn((Wheel::new(10.0), HeroesRoot));

            p.spawn(AudioBundle {
                source: asset_server.load("embedded://rockafeller_skank.ogg"),
                ..Default::default()
            });
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
                p.spawn(MaterialNodeBundle {
                    material: ui_materials.add(SuperellipseUiMaterial {
                        background_color: Color::BLACK,
                        border_radius: Vec4::splat(25.0),
                        border_color: Color::WHITE,
                        border_thickness: 2.0,
                        ..Default::default()
                    }),
                    style: Style {
                        margin: UiRect::all(Val::Percent(5.0)),
                        padding: UiRect::all(Val::Px(25.0)),
                        width: Val::Percent(20.0),
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
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
                p.spawn(MaterialNodeBundle {
                    material: ui_materials.add(SuperellipseUiMaterial {
                        background_color: Color::BLACK,
                        border_radius: Vec4::splat(25.0),
                        border_color: Color::WHITE,
                        border_thickness: 2.0,
                        ..Default::default()
                    }),
                    style: Style {
                        margin: UiRect::axes(Val::Percent(10.0), Val::Px(50.0)),
                        padding: UiRect::all(Val::Px(25.0)),
                        width: Val::Percent(30.0),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
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
                p.spawn(MaterialNodeBundle {
                    material: ui_materials.add(SuperellipseUiMaterial {
                        background_color: Color::BLACK,
                        border_radius: Vec4::splat(25.0),
                        border_color: Color::WHITE,
                        border_thickness: 2.0,
                        ..Default::default()
                    }),
                    style: Style {
                        margin: UiRect::all(Val::Percent(5.0)),
                        padding: UiRect::all(Val::Px(25.0)),
                        width: Val::Percent(20.0),
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
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
    }
}

fn update(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut state: ResMut<State>,
    time: Res<Time>,
    mut desc_node: Query<&mut Text, (With<DescNode>, Without<NameNode>, Without<StatsNode>)>,
    mut name_node: Query<&mut Text, (Without<DescNode>, With<NameNode>, Without<StatsNode>)>,
    mut stats_node: Query<&mut Text, (Without<DescNode>, Without<NameNode>, With<StatsNode>)>,
    wheel: Query<(&Wheel, &Children)>,
    heroes: Query<&HeroComponent>,
) {
    let (wheel, children) = wheel.single();

    let selected_hero = heroes.get(*children.get(wheel.current()).unwrap()).unwrap();

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
        next_state.set(GameState::Landing);
    }

    state.timer += time.delta_seconds();
    if state.timer >= 1.0 {
        // next_state.set(GameState::Landing);
    }
}
