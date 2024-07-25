use avian3d::{dynamics::integrator::Gravity, prelude::PhysicsLayer};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Gravity::ZERO);
}

#[derive(PhysicsLayer, Clone, Copy, Debug)]
pub enum CollisionLayer {
    Asteroid,
    Enemy,
    Bullet,
}
