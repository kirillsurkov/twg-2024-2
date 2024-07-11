use bevy::prelude::*;

use super::{
    cards::{
        CardDesc, CardFooter, CardHeader, CardHolder, CardLevels, CardsControl, CardsControls,
        CardsHolder, CardsRoot,
    },
    players::PlayersRoot,
    screen::{ScreenBody, ScreenFooter, ScreenHeader, ScreenMain, ScreenRoot, ScreenSide},
    LocalSchedule,
};

#[derive(Component)]
pub struct FightHomeLayout;

impl Plugin for FightHomeLayout {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, init);
    }
}

fn init(mut commands: Commands, query: Query<Entity, Added<FightHomeLayout>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert((NodeBundle::default(), ScreenRoot))
            .with_children(|p| {
                p.spawn((NodeBundle::default(), ScreenMain))
                    .with_children(|p| {
                        p.spawn((NodeBundle::default(), ScreenHeader));
                        p.spawn((NodeBundle::default(), ScreenBody))
                            .with_children(|p| {
                                p.spawn((NodeBundle::default(), PlayersRoot))
                                    .with_children(|p| {
                                        p.spawn(TextBundle::from_section(
                                            "player 1",
                                            TextStyle::default(),
                                        ));
                                        p.spawn(TextBundle::from_section(
                                            "player 2",
                                            TextStyle::default(),
                                        ));
                                        p.spawn(TextBundle::from_section(
                                            "player 3",
                                            TextStyle::default(),
                                        ));
                                        p.spawn(TextBundle::from_section(
                                            "player 4",
                                            TextStyle::default(),
                                        ));
                                        p.spawn(TextBundle::from_section(
                                            "player 5",
                                            TextStyle::default(),
                                        ));
                                        p.spawn(TextBundle::from_section(
                                            "player 6",
                                            TextStyle::default(),
                                        ));
                                    });
                                p.spawn((NodeBundle::default(), CardsRoot))
                                    .with_children(|p| {
                                        p.spawn((NodeBundle::default(), CardsHolder))
                                            .with_children(|p| {
                                                p.spawn((NodeBundle::default(), CardHolder))
                                                    .with_children(|p| {
                                                        p.spawn((
                                                            NodeBundle::default(),
                                                            CardHeader,
                                                        ));
                                                        p.spawn((
                                                            NodeBundle::default(),
                                                            CardLevels,
                                                        ));
                                                        p.spawn((
                                                            CardDesc,
                                                            TextBundle::from_section(
                                                                "Card 1",
                                                                TextStyle::default(),
                                                            ),
                                                        ));
                                                        p.spawn((
                                                            NodeBundle::default(),
                                                            CardFooter,
                                                        ));
                                                    });
                                                p.spawn((NodeBundle::default(), CardHolder))
                                                    .with_children(|p| {
                                                        p.spawn((
                                                            NodeBundle::default(),
                                                            CardHeader,
                                                        ));
                                                        p.spawn((
                                                            NodeBundle::default(),
                                                            CardLevels,
                                                        ));
                                                        p.spawn((
                                                            CardDesc,
                                                            TextBundle::from_section(
                                                                "Card 2",
                                                                TextStyle::default(),
                                                            ),
                                                        ));
                                                        p.spawn((
                                                            NodeBundle::default(),
                                                            CardFooter,
                                                        ));
                                                    });
                                                p.spawn((NodeBundle::default(), CardHolder))
                                                    .with_children(|p| {
                                                        p.spawn((
                                                            NodeBundle::default(),
                                                            CardHeader,
                                                        ));
                                                        p.spawn((
                                                            NodeBundle::default(),
                                                            CardLevels,
                                                        ));
                                                        p.spawn((
                                                            CardDesc,
                                                            TextBundle::from_section(
                                                                "Card 3",
                                                                TextStyle::default(),
                                                            ),
                                                        ));
                                                        p.spawn((
                                                            NodeBundle::default(),
                                                            CardFooter,
                                                        ));
                                                    });
                                            });
                                        p.spawn((NodeBundle::default(), CardsControls))
                                            .with_children(|p| {
                                                p.spawn((
                                                    CardsControl,
                                                    TextBundle::from_section(
                                                        "control 1",
                                                        TextStyle {
                                                            font_size: 25.0,
                                                            ..Default::default()
                                                        },
                                                    ),
                                                ));
                                                p.spawn((
                                                    CardsControl,
                                                    TextBundle::from_section(
                                                        "control 2",
                                                        TextStyle {
                                                            font_size: 25.0,
                                                            ..Default::default()
                                                        },
                                                    ),
                                                ));
                                                p.spawn((
                                                    CardsControl,
                                                    TextBundle::from_section(
                                                        "control 3",
                                                        TextStyle {
                                                            font_size: 25.0,
                                                            ..Default::default()
                                                        },
                                                    ),
                                                ));
                                                p.spawn((
                                                    CardsControl,
                                                    TextBundle::from_section(
                                                        "control 4",
                                                        TextStyle {
                                                            font_size: 25.0,
                                                            ..Default::default()
                                                        },
                                                    ),
                                                ));
                                            });
                                    });
                            });
                        p.spawn((NodeBundle::default(), ScreenFooter));
                    });
                p.spawn((NodeBundle::default(), ScreenSide));
            });
    }
}
