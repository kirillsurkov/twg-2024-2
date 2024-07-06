use std::f32::consts::FRAC_PI_2;

use bevy::{core_pipeline::bloom::BloomSettings, pbr::NotShadowCaster, prelude::*};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_hanabi::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_raycast::immediate::{Raycast, RaycastSettings};
use complex_anim_player::ComplexAnimPlayerPlugin;
use hero::{dimas::Dimas, dtyan::DTyan, duck::Duck, nulch::Nulch, rasp::Rasp, HeroPlugin};
use wheel::{Wheel, WheelPlugin};

mod complex_anim_player;
mod hero;
mod wheel;

#[derive(Component)]
struct Laser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HanabiPlugin)
        .add_plugins(NoCameraPlayerPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(
            HeroPlugin::default()
                .with_hero::<Nulch>()
                .with_hero::<Rasp>()
                .with_hero::<DTyan>()
                .with_hero::<Dimas>()
                .with_hero::<Duck>(),
        )
        .add_plugins(WheelPlugin)
        .add_plugins(ComplexAnimPlayerPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(Update, laser)
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
        FlyCam,
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

    commands.spawn(Wheel::new(10.0)).with_children(|p| {
        p.spawn(Nulch);
        p.spawn(Rasp);
        p.spawn(DTyan);
        p.spawn(Dimas);
        p.spawn(Duck);
    });

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
