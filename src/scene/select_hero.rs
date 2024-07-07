use bevy::{core_pipeline::bloom::BloomSettings, pbr::NotShadowCaster, prelude::*};

use crate::{component::wheel::Wheel, hero::HeroesRoot, scene::landing::HeroSelected};

use super::{GameState, InitScene, Root};

#[derive(Resource)]
pub struct State {
    timer: f32,
}

pub fn update(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    state: Option<ResMut<State>>,
    time: Res<Time>,
    query: Query<Entity, (With<Root>, Added<InitScene>)>,
    wheel: Query<(&Wheel, &Children)>,
    named: Query<&Name>,
) {
    for root in query.iter() {
        println!("SELECT HERO INIT");
        commands.insert_resource(State { timer: 0.0 });
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

            p.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                    material: materials.add(StandardMaterial {
                        base_color: Color::hex("888888").unwrap().into(),
                        unlit: true,
                        cull_mode: None,
                        ..default()
                    }),
                    transform: Transform::from_scale(Vec3::splat(20.0))
                        .with_translation(Vec3::new(0.0, 0.0, 5.0)),
                    ..default()
                },
                NotShadowCaster,
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
        });
        return;
    }

    let selected = {
        let (wheel, children) = wheel.single();
        if wheel.selected() {
            let selected = children.get(wheel.current()).unwrap();
            named.get(*selected).ok().map(Name::as_str)
        } else {
            None
        }
    };

    if let Some(selected) = selected {
        commands.insert_resource(HeroSelected {
            name: selected.to_string(),
        });
        next_state.set(GameState::Landing);
    }

    let mut state = state.unwrap();

    state.timer += time.delta_seconds();
    if state.timer >= 1.0 {
        // next_state.set(GameState::Landing);
    }
}
