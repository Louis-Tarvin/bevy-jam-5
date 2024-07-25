use avian3d::{
    collision::contact_reporting::Collision, dynamics::integrator::Gravity, prelude::PhysicsLayer,
};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, print_collisions);
    app.insert_resource(Gravity::ZERO);
}

#[derive(PhysicsLayer, Clone, Copy, Debug)]
pub enum CollisionLayer {
    Asteroid,
    Enemy,
    Bullet,
}

fn print_collisions(mut collision_event_reader: EventReader<Collision>) {
    for Collision(contacts) in collision_event_reader.read() {
        println!(
            "Entities {:?} and {:?} are colliding",
            contacts.entity1, contacts.entity2,
        );
    }
}
