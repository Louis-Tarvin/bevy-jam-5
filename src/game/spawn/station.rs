use bevy::prelude::*;
use bevy_health_bar3d::configuration::{BarHeight, BarSettings};

use crate::{
    game::{
        assets::{HandleMap, ObjectKey},
        util::Spin,
    },
    screen::Screen,
};

use super::building::Destructable;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_station);
}

#[derive(Event, Debug)]
pub struct SpawnStation;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Station;

#[derive(Component)]
pub struct StationCameraTarget;

fn spawn_station(
    _trigger: Trigger<SpawnStation>,
    mut commands: Commands,
    object_handles: Res<HandleMap<ObjectKey>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut transform = Transform::from_xyz(0.0, 0.0, -30.0);
    transform.rotate_z(f32::to_radians(90.0));
    transform.rotate_y(f32::to_radians(20.0));
    transform.scale = Vec3::splat(2.8);
    commands
        .spawn((
            Name::new("Station"),
            SceneBundle {
                scene: object_handles[&ObjectKey::Station].clone_weak(),
                transform,
                ..Default::default()
            },
            Station,
            Spin {
                rotation_axis: transform.rotation * Vec3::Y,
                rotation_speed: 0.05,
            },
            Destructable::new(1000.0),
            StateScoped(Screen::Playing),
            BarSettings::<Destructable> {
                width: 10.0,
                offset: 11.0,
                height: BarHeight::Static(1.0),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            // spawn random lights along the radius
            let mesh = meshes.add(Sphere { radius: 0.1 }.mesh().ico(1).unwrap());
            let material = materials.add(StandardMaterial {
                base_color: Color::srgb(4.0, 4.0, 4.0),
                ..Default::default()
            });
            for _ in 0..150 {
                let angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
                let z = rand::random::<f32>() - 0.5;
                let position = Vec3::new(angle.sin() * 9.5, z, angle.cos() * 9.5);
                parent.spawn((
                    Name::new("Light"),
                    PbrBundle {
                        mesh: mesh.clone(),
                        material: material.clone(),
                        transform: Transform::from_translation(position),
                        ..Default::default()
                    },
                ));
            }
        });

    commands.spawn((
        Name::new("CameraTarget"),
        Transform::from_translation(Vec3::new(0.0, 0.0, 200.0)),
        GlobalTransform::default(),
        StationCameraTarget,
    ));
}
