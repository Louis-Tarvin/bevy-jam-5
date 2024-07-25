use avian3d::{
    collision::{Collider, CollisionLayers, LayerMask},
    dynamics::rigid_body::{LinearVelocity, RigidBody},
};
use bevy::prelude::*;

use crate::{
    game::{collision::CollisionLayer, combat::ShootEvent},
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_bullet);
    app.init_resource::<BulletAssets>();
    app.add_systems(Update, destroy_after_secs.in_set(AppSet::TickTimers));
}

#[derive(Resource, Default)]
pub struct BulletAssets {
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<Handle<StandardMaterial>>,
}

fn spawn_bullet(
    trigger: Trigger<ShootEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut bullet_assets: ResMut<BulletAssets>,
) {
    let event = trigger.event();
    let direction = event.direction.try_normalize().unwrap_or(Vec3::X);

    if bullet_assets.mesh.is_none() {
        bullet_assets.mesh = Some(meshes.add(Sphere { radius: 0.5 }.mesh().ico(1).unwrap()));
    }
    if bullet_assets.material.is_none() {
        bullet_assets.material = Some(materials.add(StandardMaterial {
            base_color: Color::srgb(2.0, 4.0, 2.0),
            ..Default::default()
        }));
    }

    commands.spawn((
        Name::new("Bullet"),
        PbrBundle {
            mesh: bullet_assets.mesh.clone().unwrap(),
            material: bullet_assets.material.clone().unwrap(),
            transform: Transform::from_translation(event.position),
            ..Default::default()
        },
        LinearVelocity(direction * 100.0),
        RigidBody::Dynamic,
        Collider::sphere(0.5),
        CollisionLayers::new(
            [CollisionLayer::Bullet],
            LayerMask::from(CollisionLayer::Enemy),
        ),
        DestroyAfterSecs::new(5.0),
    ));
}

#[derive(Component)]
struct DestroyAfterSecs {
    timer: Timer,
}
impl DestroyAfterSecs {
    fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

fn destroy_after_secs(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DestroyAfterSecs)>,
) {
    for (entity, mut destroy_after_secs) in query.iter_mut() {
        destroy_after_secs.timer.tick(time.delta());
        if destroy_after_secs.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
