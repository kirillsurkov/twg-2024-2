use std::{
    f32::consts::{FRAC_PI_2, PI},
    time::Duration,
};

use bevy::{core_pipeline::bloom::BloomSettings, pbr::NotShadowCaster, prelude::*};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_hanabi::prelude::*;
use bevy_mod_raycast::immediate::{Raycast, RaycastSettings};
use hero::{
    derevotyan::Derevotyan, dimas::Dimas, duck::Duck, nulch::Nulch, rasp::Rasp, HeroPlugin,
};

mod hero;

#[derive(Component)]
struct Laser;

#[derive(Component)]
struct Scroll {
    current: u32,
    max: u32,
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>, usize);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HanabiPlugin)
        .add_plugins(NoCameraPlayerPlugin)
        .add_plugins(
            HeroPlugin::default()
                .with_hero::<Nulch>()
                .with_hero::<Rasp>()
                .with_hero::<Derevotyan>()
                .with_hero::<Dimas>()
                .with_hero::<Duck>(),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(Update, (laser, scroll))
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // commands.spawn(Nulch::default());

    commands.insert_resource(Animations(
        vec![
            asset_server.load("rasp.glb#Animation0"),
            asset_server.load("rasp.glb#Animation1"),
            asset_server.load("rasp.glb#Animation2"),
        ],
        0,
    ));

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 2.0, 5.0))
                .looking_at(Vec3::new(0.0, 1.5, 0.0), Vec3::Y),
            ..default()
        },
        BloomSettings::default(),
        // FogSettings {
        //     color: Color::rgba(0.35, 0.35, 0.35, 1.0),
        //     directional_light_color: Color::rgba(1.0, 0.95, 0.85, 0.5),
        //     directional_light_exponent: 30.0,
        //     falloff: FogFalloff::from_visibility_colors(
        //         30.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
        //         Color::rgb(0.35, 0.35, 0.35), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
        //         Color::rgb(0.8, 0.8, 0.8), // atmospheric inscattering color (light gained due to scattering from the sun)
        //     ),
        // },
        // FlyCam,
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(2.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("888888").unwrap().into(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(10.0)),
            ..default()
        },
        NotShadowCaster,
    ));

    commands.spawn(DirectionalLightBundle {
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

    let n = 5;
    let r = 10.0;

    commands
        .spawn((
            TransformBundle {
                local: Transform {
                    translation: Vec3::new(0.0, 0.0, -r),
                    ..Default::default()
                },
                ..Default::default()
            },
            VisibilityBundle::default(),
            Scroll { current: 0, max: n },
        ))
        .with_children(|p| {
            for i in 0..n {
                let ang = 2.0 * PI * i as f32 / n as f32;
                let x = ang.sin() * r;
                let y = ang.cos() * r;
                let transform = TransformBundle {
                    local: Transform::from_translation(Vec3::new(x, 0.0, y))
                        .with_rotation(Quat::from_rotation_y(ang)),
                    ..Default::default()
                };
                match i {
                    0 => p.spawn((Nulch, transform, VisibilityBundle::default())),
                    1 => p.spawn((Rasp, transform, VisibilityBundle::default())),
                    2 => p.spawn((Derevotyan, transform, VisibilityBundle::default())),
                    3 => p.spawn((Dimas, transform, VisibilityBundle::default())),
                    4 => p.spawn((Duck, transform, VisibilityBundle::default())),
                    _ => unreachable!(),
                };
            }
        });

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Sphere { radius: 1.0 }),
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::BLACK,
    //         unlit: true,
    //         ..Default::default()
    //     }),
    //     ..Default::default()
    // });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder {
                half_height: 0.5,
                radius: 0.01,
            }),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(4.0, 0.0, 0.0),
                unlit: true,
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
            ..Default::default()
        },
        Laser,
    ));

    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(10.0, 10.0, 0.0, 1.0));
    color_gradient.add_key(0.5, Vec4::new(10.0, 0.0, 0.0, 1.0));
    color_gradient.add_key(1.0, Vec4::ZERO);

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec2::new(0.1, 0.02));
    size_gradient.add_key(1.0, Vec2::new(0.1, 0.0));

    let writer = ExprWriter::new();

    let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.0).expr());
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, writer.lit(0.1).expr());

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::new(0.0, -0.01, 0.0)).expr(),
        radius: writer.lit(0.0).expr(),
        dimension: ShapeDimension::Volume,
    };

    let normal = writer.lit(Vec3::new(0.0, 1.0, 0.0));

    let tangent = writer
        .lit(Vec3::ONE)
        .uniform(writer.lit(Vec3::NEG_ONE))
        .cross(normal.clone())
        .normalized();
    let velocity = (normal + tangent * writer.lit(0.5).uniform(writer.lit(1.5)))
        * writer.lit(1.0).uniform(writer.lit(2.0));

    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, velocity.expr());

    let effect = effects.add(
        EffectAsset::new(vec![1024], Spawner::rate(128.0.into()), writer.finish())
            .init(init_pos)
            // Make spawned particles move away from the emitter origin
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            //.update(update_accel1)
            //.render_groups(RibbonModifier, trail)
            .render(ColorOverLifetimeModifier {
                gradient: color_gradient,
            })
            .render(OrientModifier {
                mode: OrientMode::FaceCameraPosition,
                rotation: None,
            })
            .render(SizeOverLifetimeModifier {
                gradient: size_gradient,
                screen_space_size: false,
            })
            .render(OrientModifier::new(OrientMode::AlongVelocity)),
    );

    commands.spawn((ParticleEffectBundle {
        effect: ParticleEffect::new(effect),
        transform: Transform::IDENTITY,
        ..Default::default()
    },));
}

fn laser(
    mut raycast: Raycast,
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut laser: Query<(Entity, &mut Transform), With<Laser>>,
    mut effect: Query<&mut Transform, (With<ParticleEffect>, Without<Laser>)>,
    mut spawner: Query<&mut EffectSpawner, Without<Laser>>,
) {
    let (laser, mut laser_transform) = laser.single_mut();
    let Ok(mut spawner) = spawner.get_single_mut() else {
        return;
    };
    let Ok(mut effect_transform) = effect.get_single_mut() else {
        return;
    };

    let factor = (time.elapsed_seconds() % 5.0) / 5.0;
    let pos1 = Vec3::new(-6.0, 10.0, 0.0);
    let mut pos2 = Vec3::new(-2.0 + factor * 4.0, 0.0, 0.0);
    let dir = (pos2 - pos1).normalize();
    if let Some((_, hit)) = raycast
        .cast_ray(
            Ray3d::new(pos1, dir),
            &RaycastSettings::default().with_filter(&|e| e != laser),
        )
        .first()
    {
        pos2 = hit.position();
        *effect_transform = Transform::from_translation(pos2).looking_to(hit.normal(), Vec3::Y);
        spawner.set_active(true);
    } else {
        pos2 = pos1 + dir * 100.0;
        spawner.set_active(false);
    }

    let len = pos1.distance(pos2);

    *laser_transform = Transform::from_translation((pos1 + pos2) / 2.0)
        .with_scale(Vec3::new(1.0, len, 1.0))
        .looking_at(pos2, Vec3::Y);
    laser_transform.rotate_z(FRAC_PI_2);
}

fn anim(
    mut animations: ResMut<Animations>,
    mut players: Query<&mut AnimationPlayer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let anim = if keyboard_input.just_pressed(KeyCode::Digit1) {
        Some(0)
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        Some(1)
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        Some(2)
    } else {
        None
    };

    for mut player in &mut players {
        if let Some(anim) = anim {
            animations.1 = anim;
            player
                .play_with_transition(
                    animations.0[animations.1].clone_weak(),
                    Duration::from_millis(250),
                )
                .repeat();
        }
    }
}

fn scroll(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Scroll, &mut Transform)>,
    time: Res<Time>,
) {
    let Ok((mut scroll, mut transform)) = query.get_single_mut() else {
        return;
    };

    let ang = -2.0 * PI * scroll.current as f32 / scroll.max as f32;

    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        scroll.current = (scroll.current + scroll.max - 1) % scroll.max;
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        scroll.current = (scroll.current + 1) % scroll.max;
    }

    transform.rotation = transform
        .rotation
        .slerp(Quat::from_rotation_y(ang), 10.0 * time.delta_seconds());
}
