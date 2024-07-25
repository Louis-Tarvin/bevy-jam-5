//! Handle player input and translate it into movement.

use bevy::prelude::*;

use crate::AppSet;

use super::spawn::asteroid::Asteroid;

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        record_movement_controller.in_set(AppSet::RecordInput),
    );

    // Apply movement based on controls.
    app.register_type::<Velocity>();
    app.add_systems(
        Update,
        ((update_velocity, apply_velocity).chain(), asteroid_rotation).in_set(AppSet::Update),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController {
    thrust_multiplier: f32,
    friction: f32,
    velocity_limit: f32,
    thrust: Vec2,
}
impl MovementController {
    pub fn new(thrust_multiplier: f32, friction: f32, velocity_limit: f32) -> Self {
        Self {
            thrust_multiplier,
            friction,
            velocity_limit,
            ..Default::default()
        }
    }
}

fn record_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // Normalize so that diagonal movement has the same speed as
    // horizontal and vertical movement.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.thrust = intent;
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Velocity(pub Vec2);

fn update_velocity(mut query: Query<(&MovementController, &mut Velocity)>, time: Res<Time>) {
    for (controller, mut velocity) in query.iter_mut() {
        // Apply friction.
        velocity.0 *= 1.0 - controller.friction * time.delta_seconds();

        // Apply thrust.
        velocity.0 += controller.thrust * controller.thrust_multiplier * time.delta_seconds();

        // Limit max velocity.
        if velocity.0.length() > controller.velocity_limit {
            velocity.0 = velocity.0.normalize() * controller.velocity_limit;
        }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += Vec3::new(velocity.0.x, velocity.0.y, 0.0) * time.delta_seconds();
    }
}

const ASTEROID_ROTATION_SPEED: f32 = 0.1;

fn asteroid_rotation(mut query: Query<(&Asteroid, &mut Transform)>, time: Res<Time>) {
    for (asteroid, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_axis_angle(
            asteroid.rotation_axis,
            time.delta_seconds() * ASTEROID_ROTATION_SPEED,
        ));
    }
}
