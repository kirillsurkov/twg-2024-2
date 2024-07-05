use bevy::prelude::*;

use super::{Hero, SelectWheel};

#[derive(Component)]
pub struct DTyan;

impl Hero for DTyan {
    fn register(app: &mut App) {
        app.add_systems(Update, (on_add, on_select_wheel));
    }
}

fn on_add(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, Added<DTyan>>,
) {
    for entity in query.iter() {
        commands.entity(entity).with_children(|p| {
            p.spawn(SceneBundle {
                scene: asset_server.load("derevotyan.glb#Scene0"),
                transform: Transform::from_scale(Vec3::splat(0.1)),
                ..Default::default()
            });
        });
    }
}

fn on_select_wheel(query: Query<&DTyan, With<SelectWheel>>) {
    for hero in query.iter() {}
}
