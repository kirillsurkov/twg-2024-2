use bevy::{
    core_pipeline::bloom::BloomSettings,
    prelude::*,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    utils::hashbrown::HashMap,
};

use crate::{
    battle_bridge::{HeroesResource, RoundCaptureResource},
    hero::{HeroId, HeroesRoot},
};

use super::{landing::HeroWatch, InvalidateTree, LocalSchedule};

pub struct AvatarsPlugin;

impl Plugin for AvatarsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            LocalSchedule,
            (
                init,
                move_to_layer,
                update_thumbnails,
                update_home.run_if(not(resource_exists::<RoundCaptureResource>)),
                update_fight.run_if(resource_exists::<RoundCaptureResource>),
            ),
        );
    }
}

#[derive(Resource)]
pub struct AvatarsResource {
    pub left: Handle<Image>,
    pub right: Handle<Image>,
    pub thumbnails: HashMap<String, Handle<Image>>,
    current_thumbnail: usize,
}

pub enum AvatarLocation {
    Thumbnail,
    Portrait,
}

#[derive(Component)]
pub struct HeroState {
    pub location: AvatarLocation,
    pub camera: Transform,
}

fn image() -> Image {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..Default::default()
    };
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..Default::default()
    };
    image.resize(size);
    image
}

#[derive(Component)]
enum Avatar {
    Thumbnail,
    Left,
    Right,
}

#[derive(Component)]
struct ThumbnailCamera;

#[derive(Component)]
struct PortraitLeftCamera;

#[derive(Component)]
struct PortraitRightCamera;

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>, heroes: Res<HeroesResource>) {
    let camera = |image_handle: Handle<Image>| Camera3dBundle {
        camera: Camera {
            hdr: true,
            order: 1,
            target: image_handle.into(),
            clear_color: Color::MIDNIGHT_BLUE.into(),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 5.0, 5.0))
            .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
        ..Default::default()
    };

    let image_left_handle = images.add(image());
    let image_right_handle = images.add(image());
    let mut thumbnails = HashMap::new();
    for (hero, _) in heroes.iter() {
        thumbnails.insert(hero.id.to_string(), images.add(image()));
    }

    commands.insert_resource(AvatarsResource {
        left: image_left_handle.clone(),
        right: image_right_handle.clone(),
        thumbnails,
        current_thumbnail: 0,
    });

    commands.spawn((
        camera(images.add(image())),
        ThumbnailCamera,
        BloomSettings::default(),
        RenderLayers::layer(1),
    ));

    commands.spawn((
        camera(image_left_handle),
        PortraitLeftCamera,
        BloomSettings::default(),
        RenderLayers::layer(2),
    ));

    commands.spawn((
        camera(image_right_handle),
        PortraitRightCamera,
        BloomSettings::default(),
        RenderLayers::layer(3),
    ));

    commands.spawn((HeroesRoot, Avatar::Thumbnail));
    commands.spawn((HeroesRoot, Avatar::Left));
    commands.spawn((HeroesRoot, Avatar::Right));

    let light = |layer| {
        (
            DirectionalLightBundle {
                directional_light: DirectionalLight {
                    color: Color::rgb(0.98, 0.95, 0.82),
                    shadows_enabled: true,
                    illuminance: 1000.0,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .looking_at(Vec3::new(0.15, -0.15, -0.25), Vec3::Y),
                ..Default::default()
            },
            RenderLayers::layer(layer),
        )
    };

    commands.spawn(light(1));
    commands.spawn(light(2));
    commands.spawn(light(3));
}

fn init(mut commands: Commands, mut query: Query<(Entity, &Children, &Avatar), Added<Avatar>>) {
    for (entity, children, avatar) in query.iter_mut() {
        println!("INIT AVATARS");
        for hero in children.iter() {
            commands.entity(*hero).insert((
                HeroState {
                    camera: Transform::default(),
                    location: match avatar {
                        Avatar::Thumbnail => AvatarLocation::Thumbnail,
                        _ => AvatarLocation::Portrait,
                    },
                },
                TransformBundle::default(),
                VisibilityBundle {
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
            ));
        }

        commands
            .entity(entity)
            .insert((TransformBundle::default(), VisibilityBundle::default()));
    }
}

fn move_to_layer(
    mut commands: Commands,
    query: Query<(Entity, &Avatar), With<InvalidateTree>>,
    children_query: Query<&Children>,
) {
    for (entity, avatar) in query.iter() {
        for child in children_query.iter_descendants(entity) {
            commands
                .entity(child)
                .insert(RenderLayers::layer(match avatar {
                    Avatar::Thumbnail => 1,
                    Avatar::Left => 2,
                    Avatar::Right => 3,
                }));
        }
    }
}

fn update_thumbnails(
    mut commands: Commands,
    mut avatars: ResMut<AvatarsResource>,
    mut camera: Query<(&mut Camera, &mut Transform), With<ThumbnailCamera>>,
    heroes: Query<(&Avatar, &Children)>,
    hero: Query<(&HeroId, &HeroState)>,
) {
    let (mut camera, mut camera_transform) = camera.single_mut();

    for (avatar, children) in heroes.iter() {
        match avatar {
            Avatar::Thumbnail => {}
            _ => continue,
        };

        let Ok((id, state)) = hero.get(children[avatars.current_thumbnail]) else {
            continue;
        };

        camera.target = avatars.thumbnails[&id.0].clone_weak().into();
        *camera_transform = state.camera;

        for (i, hero) in children.iter().enumerate() {
            commands
                .entity(*hero)
                .insert(if i == avatars.current_thumbnail {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                });
        }

        avatars.current_thumbnail = (avatars.current_thumbnail + 1) % avatars.thumbnails.len();
    }
}

fn update_home(
    mut commands: Commands,
    mut camera: Query<&mut Transform, With<PortraitRightCamera>>,
    query: Query<(&Avatar, &Children)>,
    hero: Query<(&HeroId, &HeroState)>,
    watch: Res<HeroWatch>,
) {
    let mut camera = camera.single_mut();

    for (avatar, children) in query.iter() {
        match avatar {
            Avatar::Right => {}
            _ => continue,
        }
        for child in children {
            let Ok((id, state)) = hero.get(*child) else {
                continue;
            };
            commands.entity(*child).insert(if id.0 == watch.id {
                *camera = state.camera;
                Visibility::Inherited
            } else {
                Visibility::Hidden
            });
        }
    }
}

fn update_fight(
    mut commands: Commands,
    mut camera_left: Query<
        &mut Transform,
        (With<PortraitLeftCamera>, Without<PortraitRightCamera>),
    >,
    mut camera_right: Query<
        &mut Transform,
        (With<PortraitRightCamera>, Without<PortraitLeftCamera>),
    >,
    query: Query<(&Avatar, &Children)>,
    hero: Query<(&HeroId, &HeroState)>,
    watch: Res<HeroWatch>,
    round: Res<RoundCaptureResource>,
) {
    let mut camera_left = camera_left.single_mut();
    let mut camera_right = camera_right.single_mut();

    for (avatar, children) in query.iter() {
        match avatar {
            Avatar::Thumbnail => continue,
            _ => {}
        }
        let round = round.by_player(&watch.id).unwrap();
        for child in children {
            let Ok((id, state)) = hero.get(*child) else {
                continue;
            };

            let current_id = match avatar {
                Avatar::Left => round.player1,
                Avatar::Right => round.player2,
                _ => unreachable!(),
            };

            commands.entity(*child).insert(if id.0 == current_id {
                match avatar {
                    Avatar::Left => *camera_left = state.camera,
                    Avatar::Right => *camera_right = state.camera,
                    _ => unreachable!(),
                };
                Visibility::Inherited
            } else {
                Visibility::Hidden
            });
        }
    }
}
