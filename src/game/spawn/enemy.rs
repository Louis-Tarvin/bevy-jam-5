use avian3d::{
    collision::{Collider, CollisionLayers, LayerMask, Sensor},
    dynamics::rigid_body::RigidBody,
};
use bevy::prelude::*;

use crate::{
    game::{
        assets::{HandleMap, ObjectKey},
        collision::CollisionLayer,
    },
    screen::Screen,
    AppSet,
};

use super::building::Destructable;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_enemy);
    app.register_type::<Enemy>();
    app.add_systems(
        Update,
        (choose_target, travel_to_target, attack_target)
            .chain()
            .in_set(AppSet::Update),
    );
}

#[derive(Event, Debug)]
pub struct SpawnEnemy {
    pub position: Vec3,
}

#[derive(Default, Debug, Reflect)]
pub enum EnemyState {
    #[default]
    None,
    Wander(Vec2),
    TravelingTo(Entity),
    Attacking(Entity),
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy {
    pub health: f32,
    pub state: EnemyState,
}
impl Enemy {
    pub fn new() -> Self {
        Self {
            health: 100.0,
            state: EnemyState::None,
        }
    }
}

fn spawn_enemy(
    trigger: Trigger<SpawnEnemy>,
    mut commands: Commands,
    object_handles: Res<HandleMap<ObjectKey>>,
) {
    commands.spawn((
        Name::new("Enemy"),
        Enemy::new(),
        SceneBundle {
            scene: object_handles[&ObjectKey::Enemy].clone_weak(),
            transform: Transform::from_translation(trigger.event().position),
            ..Default::default()
        },
        Collider::sphere(1.0),
        CollisionLayers::new(
            [CollisionLayer::Enemy],
            LayerMask::from([CollisionLayer::Bullet, CollisionLayer::Turret]),
        ),
        Sensor,
        RigidBody::Kinematic,
        StateScoped(Screen::Playing),
    ));
}

const WANDER_PROBABILITY: f32 = 0.7;

fn choose_target(
    mut enemy_query: Query<(&Transform, &mut Enemy), Without<Destructable>>,
    building_query: Query<(Entity, &Transform), With<Destructable>>,
) {
    for (enemy_transform, mut enemy) in enemy_query.iter_mut() {
        if let EnemyState::None = enemy.state {
            if rand::random::<f32>() < WANDER_PROBABILITY {
                let angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
                let direction = Vec2::new(angle.cos(), angle.sin());
                let distance = rand::random::<f32>() * 20.0;
                let target =
                    Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y)
                        + direction * distance;
                enemy.state = EnemyState::Wander(target);
            } else {
                let mut closest_building = None;
                let mut closest_distance = f32::MAX;
                for (entity, building_transform) in building_query.iter() {
                    let distance = enemy_transform
                        .translation
                        .distance(building_transform.translation);
                    if distance < closest_distance {
                        closest_distance = distance;
                        closest_building = Some(entity);
                    }
                }
                if let Some(entity) = closest_building {
                    enemy.state = EnemyState::TravelingTo(entity);
                }
            }
        }
    }
}

const ATTACK_DISTANCE: f32 = 10.0;

fn travel_to_target(
    mut enemy_query: Query<(&mut Transform, &mut Enemy), Without<Destructable>>,
    building_query: Query<&Transform, With<Destructable>>,
    time: Res<Time>,
) {
    for (mut enemy_transform, mut enemy) in enemy_query.iter_mut() {
        match enemy.state {
            EnemyState::TravelingTo(target) => {
                if let Ok(target_transform) = building_query.get(target) {
                    let direction =
                        target_transform.translation.xy() - enemy_transform.translation.xy();
                    let distance = direction.length();
                    let velocity = 10.0;
                    let movement = direction.normalize() * velocity * time.delta_seconds();
                    if distance < ATTACK_DISTANCE {
                        enemy.state = EnemyState::Attacking(target);
                    } else {
                        enemy_transform.translation += movement.extend(0.0);
                    }
                } else {
                    enemy.state = EnemyState::None;
                }
            }
            EnemyState::Wander(target) => {
                let direction = target
                    - Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y);
                let distance = direction.length();
                let velocity = 7.0;
                let movement = direction.normalize() * velocity * time.delta_seconds();
                if distance < 0.1 {
                    enemy.state = EnemyState::None;
                } else {
                    enemy_transform.translation += movement.extend(0.0);
                }
            }
            _ => {}
        }
    }
}

fn attack_target(
    mut enemy_query: Query<&mut Enemy, Without<Destructable>>,
    mut building_query: Query<&mut Destructable, With<Destructable>>,
    time: Res<Time>,
) {
    for mut enemy in enemy_query.iter_mut() {
        if let EnemyState::Attacking(target) = enemy.state {
            if let Ok(mut target) = building_query.get_mut(target) {
                let damage = 10.0;
                target.health -= damage * time.delta_seconds();
            } else {
                enemy.state = EnemyState::None;
            }
        }
    }
}
