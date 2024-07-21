use avian3d::prelude::PhysicsLayer;

#[derive(PhysicsLayer, Clone, Copy, Debug)]
pub enum CollisionLayer {
    Asteroid,
    Enemy,
}
