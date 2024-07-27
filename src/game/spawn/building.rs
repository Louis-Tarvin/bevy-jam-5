use avian3d::{
    collision::{Collider, CollisionLayers, LayerMask, Sensor},
    dynamics::rigid_body::RigidBody,
};
use bevy::prelude::*;
use bevy_health_bar3d::configuration::{BarHeight, BarSettings, Percentage};

use crate::{
    game::{
        assets::{HandleMap, ObjectKey},
        collision::CollisionLayer,
        turret::Turret,
    },
    screen::Screen,
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_building);
    app.register_type::<Destructable>();
    app.add_systems(Update, destroy_building.in_set(AppSet::PostUpdate));
}

#[derive(Copy, Clone, Debug, Reflect)]
pub enum BuildingType {
    Decoy,
    Turret,
}

#[derive(Event, Debug)]
pub struct SpawnBuilding {
    pub building_type: BuildingType,
    pub position: Vec3,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Destructable {
    pub health: f32,
    max_health: f32,
}
impl Destructable {
    pub fn new(health: f32) -> Self {
        debug_assert!(health > 0.0);
        Self {
            health,
            max_health: health,
        }
    }
}

impl Percentage for Destructable {
    fn value(&self) -> f32 {
        self.health / self.max_health
    }
}

fn spawn_building(
    trigger: Trigger<SpawnBuilding>,
    mut commands: Commands,
    object_handles: Res<HandleMap<ObjectKey>>,
) {
    let event = trigger.event();
    match event.building_type {
        BuildingType::Decoy => {
            commands.spawn((
                Name::new("Decoy"),
                Destructable::new(200.0),
                SceneBundle {
                    scene: object_handles[&ObjectKey::Decoy].clone_weak(),
                    transform: Transform::from_translation(event.position),
                    ..Default::default()
                },
                StateScoped(Screen::Playing),
                BarSettings::<Destructable> {
                    width: 5.0,
                    offset: 3.0,
                    height: BarHeight::Static(0.5),
                    ..Default::default()
                },
            ));
        }
        BuildingType::Turret => {
            commands.spawn((
                Name::new("Turret"),
                Turret::new(1.0),
                Destructable::new(200.0),
                SceneBundle {
                    scene: object_handles[&ObjectKey::Decoy].clone_weak(),
                    transform: Transform::from_translation(event.position),
                    ..Default::default()
                },
                StateScoped(Screen::Playing),
                BarSettings::<Destructable> {
                    width: 5.0,
                    offset: 3.0,
                    height: BarHeight::Static(0.5),
                    ..Default::default()
                },
                Collider::sphere(13.0),
                RigidBody::Static,
                Sensor,
                CollisionLayers::new(
                    [CollisionLayer::Turret],
                    LayerMask::from(CollisionLayer::Enemy),
                ),
            ));
        }
    }
}

fn destroy_building(mut commands: Commands, query: Query<(Entity, &Destructable)>) {
    for (entity, destructable) in query.iter() {
        if destructable.health <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
