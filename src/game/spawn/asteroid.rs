use avian3d::collision::{Collider, CollisionLayers, LayerMask};
use bevy::prelude::*;
use rand::Rng;

use crate::game::{
    assets::{HandleMap, ObjectKey},
    collision::CollisionLayer,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_asteroid);
}

#[derive(Event, Debug)]
pub struct SpawnAsteroid {
    pub position: Vec3,
}

#[derive(Component, Debug)]
pub struct Asteroid {
    pub rotation_axis: Vec3,
}

fn spawn_asteroid(
    trigger: Trigger<SpawnAsteroid>,
    mut commands: Commands,
    object_handles: Res<HandleMap<ObjectKey>>,
) {
    let mut rng = rand::thread_rng();
    let mut random_rotation = Quat::IDENTITY;
    random_rotation *= Quat::from_rotation_x(f32::to_radians(rng.gen_range(0.0..360.0)));
    random_rotation *= Quat::from_rotation_y(f32::to_radians(rng.gen_range(0.0..360.0)));
    random_rotation *= Quat::from_rotation_z(f32::to_radians(rng.gen_range(0.0..360.0)));
    let transform = Transform {
        translation: trigger.event().position,
        rotation: random_rotation,
        ..Default::default()
    };
    commands.spawn((
        Name::new("Asteroid"),
        SceneBundle {
            scene: object_handles[&ObjectKey::Asteroid].clone_weak(),
            transform,
            ..Default::default()
        },
        Asteroid {
            rotation_axis: Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            )
            .normalize(),
        },
        Collider::sphere(5.0),
        CollisionLayers::new([CollisionLayer::Asteroid], LayerMask::NONE),
    ));
}
