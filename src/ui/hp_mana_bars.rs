use bevy::prelude::*;

use crate::{battle::fight::Owner, component::fight_state::FightState};

use super::LocalSchedule;

pub struct HpManaBarsPlugin;

impl Plugin for HpManaBarsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (
                init_root,
                init_bars_holder,
                init_bar_holder,
                init_bar,
                update_bar,
            )
                .run_if(resource_exists::<FightState>),
        );
    }
}

#[derive(Clone, Copy)]
enum BarKind {
    Hp,
    Mana,
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
                    margin: UiRect::top(Val::Auto),
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
                        height: Val::Percent(10.0),
                        column_gap: Val::Percent(10.0),
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
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((NodeBundle::default(), Bar(holder.0, holder.1)));
            });
    }
}

#[derive(Component)]
struct Bar(Owner, BarKind);

fn init_bar(mut commands: Commands, query: Query<(Entity, &Bar), Added<Bar>>) {
    for (entity, bar) in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
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

fn update_bar(mut query: Query<(&Bar, &mut Style)>, fight_state: Res<FightState>) {
    for (bar, mut style) in query.iter_mut() {
        let fighter = match bar.0 {
            Owner::Fighter1 => &fight_state.fighter1,
            Owner::Fighter2 => &fight_state.fighter2,
        };
        match bar.1 {
            BarKind::Hp => {
                style.width = Val::Percent(100.0 * fighter.hp / fighter.max_hp);
            }
            BarKind::Mana => {
                style.width = Val::Percent(fighter.mana);
            }
        }
    }
}
