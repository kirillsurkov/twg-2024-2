use bevy::prelude::*;

use crate::wheel;

use super::Hero;

#[derive(Component)]
pub struct Duck;

impl Hero for Duck {
    fn register(app: &mut App) {
        app.add_systems(Update, (on_add, on_wheel));
    }
}

fn on_add(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, Added<Duck>>,
) {
    for entity in query.iter() {
        commands.entity(entity).with_children(|p| {
            p.spawn(SceneBundle {
                scene: asset_server.load("duck.glb#Scene0"),
                transform: Transform::from_scale(Vec3::splat(0.5)),
                ..Default::default()
            });
        });
    }
}

fn on_wheel(query: Query<&Duck, With<wheel::State>>) {
    for hero in query.iter() {}
}
