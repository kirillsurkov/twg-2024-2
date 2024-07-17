use bevy::prelude::*;

use crate::{battle::fight::Owner, component::fight_state::FightState};

use super::LocalSchedule;

const BAR_HEIGHT: f32 = 30.0;

pub struct HpManaBarsPlugin;

impl Plugin for HpManaBarsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (
                show_hide.after(init_root),
                (
                    init_root,
                    init_bars_holder,
                    init_bar_holder,
                    init_bar,
                    update_bar.after(init_bar),
                    init_bar_caption_holder,
                )
                    .run_if(resource_exists::<FightState>),
            ),
        );
    }
}

#[derive(Clone, Copy)]
enum BarKind {
    Hp,
    Mana,
}

fn show_hide(
    mut commands: Commands,
    fight: Option<Res<FightState>>,
    query: Query<Entity, With<HpManaBarsRoot>>,
) {
    for entity in query.iter() {
        if fight.is_some() {
            commands.entity(entity).insert(Visibility::Inherited);
        } else {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
}

#[derive(Component)]
pub struct HpManaBarsRoot;

fn init_root(mut commands: Commands, query: Query<Entity, Added<HpManaBarsRoot>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(50.0),
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(35.0),
                        height: Val::Px(BAR_HEIGHT * 2.0 + 10.0),
                        column_gap: Val::Px(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|p| {
                    p.spawn((NodeBundle::default(), BarsHolder(Owner::Fighter1)));
                    p.spawn((NodeBundle::default(), BarsHolder(Owner::Fighter2)));
                });
            });
    }
}

#[derive(Component)]
struct BarsHolder(Owner);

fn init_bars_holder(
    mut commands: Commands,
    query: Query<(Entity, &BarsHolder), Added<BarsHolder>>,
) {
    for (entity, holder) in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Percent(10.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((NodeBundle::default(), BarHolder(holder.0, BarKind::Hp)));
                p.spawn((NodeBundle::default(), BarHolder(holder.0, BarKind::Mana)));
            });
    }
}

#[derive(Component)]
struct BarHolder(Owner, BarKind);

fn init_bar_holder(mut commands: Commands, query: Query<(Entity, &BarHolder), Added<BarHolder>>) {
    for (entity, holder) in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Px(BAR_HEIGHT),
                    padding: UiRect::all(Val::Px(3.0)),
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((NodeBundle::default(), Bar(holder.0, holder.1)));
                p.spawn((NodeBundle::default(), BarCaptionHolder(holder.0, holder.1)));
            });
    }
}

#[derive(Component)]
struct BarCaptionHolder(Owner, BarKind);

fn init_bar_caption_holder(
    mut commands: Commands,
    query: Query<(Entity, &BarCaptionHolder), Added<BarCaptionHolder>>,
) {
    for (entity, caption) in query.iter() {
        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(BAR_HEIGHT - 6.0),
                    margin: UiRect::top(Val::Px(-BAR_HEIGHT + 6.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 16.0,
                        ..Default::default()
                    },
                ));
            });
    }
}

#[derive(Component)]
struct BarCaption(Owner, BarKind);

fn update_bar_caption(mut query: Query<(&mut Text, &BarCaption)>, fight_state: Res<FightState>) {
    for (text, BarCaption(owner, kind)) in query.iter_mut() {
        let fighter = match owner {
            Owner::Fighter1 => &fight_state.fighter1,
            Owner::Fighter2 => &fight_state.fighter2,
        };
        let text = match kind {
            BarKind::Hp => format!("{} / {}", fighter.hp, fighter.max_hp),
            BarKind::Mana => format!("{} / {}", fighter.mana, 100),
        };
    }
}

#[derive(Component)]
struct Bar(Owner, BarKind);

fn init_bar(mut commands: Commands, query: Query<(Entity, &Bar), Added<Bar>>) {
    for (entity, bar) in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: match bar.1 {
                BarKind::Hp => Color::LIME_GREEN.into(),
                BarKind::Mana => Color::NAVY.into(),
            },
            ..Default::default()
        });
    }
}

fn update_bar(mut query: Query<(&Bar, &mut Style)>, fight_state: Res<FightState>, time: Res<Time>) {
    for (bar, mut style) in query.iter_mut() {
        let fighter = match bar.0 {
            Owner::Fighter1 => &fight_state.fighter1,
            Owner::Fighter2 => &fight_state.fighter2,
        };
        let target = match bar.1 {
            BarKind::Hp => 100.0 * fighter.hp / fighter.max_hp,
            BarKind::Mana => fighter.mana,
        };
        style.width = Val::Percent(match style.width {
            Val::Percent(current) => current + (target - current) * time.delta_seconds() * 10.0,
            _ => continue,
        });
    }
}
