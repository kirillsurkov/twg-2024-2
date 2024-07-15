use bevy::{gltf::Gltf, prelude::*};
use bevy_hanabi::prelude::*;

use crate::{battle_bridge::RoundCaptureResource, hero::HeroId, scene::landing::HeroWatch};

use super::LocalSchedule;

#[derive(Component)]
pub struct Projectile {
    origin: Entity,
    target: Entity,
    eta: f32,
    timer: f32,
}

impl Projectile {
    pub fn new(origin: Entity, target: Entity, eta: f32) -> Self {
        Self {
            origin,
            target,
            eta,
            timer: 0.0,
        }
    }
}

#[derive(Component)]
pub struct ProjectileConfig {
    size: f32,
    color: Color,
    model: Option<Handle<Gltf>>,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            LocalSchedule,
            (init, update.run_if(resource_exists::<RoundCaptureResource>)),
        );
    }
}

fn init(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    query: Query<Entity, Added<Projectile>>,
) {
    for entity in query.iter() {
        let mut color_gradient = Gradient::new();
        color_gradient.add_key(0.0, Vec4::new(10.0, 10.0, 0.0, 1.0));
        color_gradient.add_key(0.5, Vec4::new(10.0, 0.0, 0.0, 1.0));
        color_gradient.add_key(1.0, Vec4::ZERO);

        let mut size_gradient = Gradient::new();
        size_gradient.add_key(0.0, Vec2::new(0.2, 0.1));
        size_gradient.add_key(1.0, Vec2::new(0.2, 0.0));

        let writer = ExprWriter::new();

        let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.0).expr());
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, writer.lit(0.2).expr());

        let init_pos = SetPositionSphereModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            radius: writer.lit(0.1).expr(),
            dimension: ShapeDimension::Volume,
        };

        let normal = writer.add_property("normal", Vec3::Y.into());

        let tangent = writer
            .lit(Vec3::ONE)
            .uniform(writer.lit(Vec3::NEG_ONE))
            .cross(writer.prop(normal))
            .normalized();
        let velocity = (writer.prop(normal) + tangent * writer.lit(0.5).uniform(writer.lit(1.5)))
            * writer.lit(1.0).uniform(writer.lit(2.0));

        let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, velocity.expr());

        let effect = EffectAsset::new(vec![1024], Spawner::rate(1024.0.into()), writer.finish())
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
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
            .render(OrientModifier::new(OrientMode::AlongVelocity));

        commands.entity(entity).insert((
            EffectSpawner::new(&effect),
            ParticleEffectBundle {
                effect: ParticleEffect::new(effects.add(effect)),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
        ));
    }
}

fn update(
    mut commands: Commands,
    mut projectiles: Query<(
        Entity,
        &HeroId,
        &mut Projectile,
        &mut EffectProperties,
        &mut EffectSpawner,
        &mut Transform,
    )>,
    capture: Res<RoundCaptureResource>,
    watch: Res<HeroWatch>,
    time: Res<Time>,
    transforms: Query<&GlobalTransform>,
) {
    let capture = capture.by_player(&watch.id).unwrap();
    for (entity, id, mut projectile, mut properties, mut spawner, mut transform) in
        projectiles.iter_mut()
    {
        if projectile.timer >= projectile.eta {
            spawner.set_active(false);
            if projectile.timer >= projectile.eta + 0.2 {
                commands.entity(entity).despawn_recursive();
                continue;
            }
        }

        if id.0 == capture.player1 || id.0 == capture.player2 {
            commands.entity(entity).insert(Visibility::Inherited);
        } else {
            commands.entity(entity).insert(Visibility::Hidden);
        }

        let origin =
            transforms.get(projectile.origin).unwrap().translation() + Vec3::new(0.0, 2.0, 0.0);
        let target =
            transforms.get(projectile.target).unwrap().translation() + Vec3::new(0.0, 1.0, 0.0);

        let dir = target - origin;
        let origin = origin + dir * (projectile.timer / projectile.eta);

        projectile.timer += time.delta_seconds();
        properties.set("normal", (dir.normalize()).into());
        *transform = Transform::from_translation(origin);
    }
}
