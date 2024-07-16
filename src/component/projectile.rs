use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::{battle_bridge::RoundCaptureResource, hero::HeroId, scene::landing::HeroWatch};

use super::LocalSchedule;

#[derive(Component)]
pub struct Projectile {
    origin: Entity,
    target: Option<Entity>,
    eta: f32,
    timer: f32,
}

impl Projectile {
    pub fn new(origin: Entity, target: Option<Entity>, eta: f32) -> Self {
        Self {
            origin,
            target,
            eta,
            timer: 0.0,
        }
    }
}

#[derive(Component, Clone)]
pub struct ProjectileConfig {
    pub transform: Transform,
    pub radius: f32,
    pub color: Color,
    pub color_end: Color,
    pub model: Option<Handle<Scene>>,
    pub model_transform: Transform,
    pub particles: u32,
}

impl Default for ProjectileConfig {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            radius: 0.0,
            color: Color::default(),
            color_end: Color::WHITE.with_a(0.0),
            model: None,
            model_transform: Transform::default(),
            particles: 1024,
        }
    }
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
    query: Query<(Entity, &ProjectileConfig), Added<Projectile>>,
) {
    for (entity, config) in query.iter() {
        let color_start = (config.color * 10.0)
            .with_a(config.color.a())
            .rgba_to_vec4();

        let color_end = (config.color_end * 1.0)
            .with_a(config.color_end.a())
            .rgba_to_vec4();

        let mut color_gradient = Gradient::new();
        color_gradient.add_key(0.0, color_start);
        color_gradient.add_key(1.0, color_end);

        let mut size_gradient = Gradient::new();
        size_gradient.add_key(0.0, Vec2::new(0.1, 0.1));
        size_gradient.add_key(1.0, Vec2::new(0.0, 0.0));

        let writer = ExprWriter::new();

        let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.0).expr());
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, writer.lit(0.2).expr());

        let init_pos = SetPositionSphereModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            radius: writer.lit(config.radius).expr(),
            dimension: ShapeDimension::Volume,
        };

        let normal = writer.add_property("normal", Vec3::Y.into());

        let tangent = writer
            .lit(Vec3::ONE)
            .uniform(writer.lit(Vec3::NEG_ONE))
            .cross(writer.prop(normal))
            .normalized();
        let velocity = (writer.prop(normal) + tangent * writer.lit(0.0).uniform(writer.lit(0.5)))
            * writer.lit(1.0).uniform(writer.lit(2.0));

        let init_vel =
            SetAttributeModifier::new(Attribute::VELOCITY, writer.lit(Vec3::ZERO).expr());

        let effect = EffectAsset::new(
            vec![config.particles * 2],
            Spawner::rate(CpuValue::Single(config.particles as f32)),
            writer.finish(),
        )
        .with_simulation_condition(SimulationCondition::Always)
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
        });
        // .render(OrientModifier::new(OrientMode::AlongVelocity));

        commands.entity(entity).insert((
            EffectSpawner::new(&effect).with_active(false),
            ParticleEffectBundle {
                effect: ParticleEffect::new(effects.add(effect)),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
        ));

        if let Some(model) = &config.model {
            commands.entity(entity).with_children(|p| {
                p.spawn((
                    model.clone_weak(),
                    TransformBundle {
                        local: config.model_transform,
                        ..Default::default()
                    },
                    VisibilityBundle::default(),
                ));
            });
        }
    }
}

fn update(
    mut commands: Commands,
    mut projectiles: Query<(
        Entity,
        Option<&Children>,
        &HeroId,
        &ProjectileConfig,
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
    for (
        entity,
        children,
        id,
        config,
        mut projectile,
        mut properties,
        mut spawner,
        mut transform,
    ) in projectiles.iter_mut()
    {
        let round = capture.by_player(&id.0).unwrap();

        spawner.set_active(projectile.timer < projectile.eta);
        if let Some(children) = children {
            commands.entity(children[0]).insert(if spawner.is_active() {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            });
        }

        if projectile.timer >= projectile.eta + 0.2 {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        if round.player1 == watch.id || round.player2 == watch.id {
            commands.entity(entity).insert(Visibility::Inherited);
        } else {
            commands.entity(entity).insert(Visibility::Hidden);
        }

        let Some(target) = projectile.target else {
            continue;
        };

        let t = transforms
            .get(projectile.origin)
            .unwrap()
            .mul_transform(config.transform)
            .compute_transform();

        let origin = t.translation;
        let target = transforms.get(target).unwrap().translation() + Vec3::new(0.0, 1.0, 0.0);

        let dir = target - origin;
        let origin = origin + dir * (projectile.timer / projectile.eta);

        projectile.timer += time.delta_seconds();
        properties.set("normal", (dir.normalize()).into());
        *transform = Transform::from_translation(origin)
            .with_scale(t.scale)
            .with_rotation(Transform::IDENTITY.looking_to(dir, Vec3::Y).rotation * t.rotation);
    }
}
