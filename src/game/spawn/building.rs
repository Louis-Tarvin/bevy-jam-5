use bevy::prelude::*;

use crate::{
    game::assets::{HandleMap, ObjectKey},
    screen::Screen,
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_building);
    app.register_type::<Destructable>();
    app.add_systems(Update, destroy_building.in_set(AppSet::PostUpdate));
}

#[derive(Copy, Clone, Debug)]
pub enum BuildingType {
    Decoy,
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
                Destructable { health: 100.0 },
                SceneBundle {
                    scene: object_handles[&ObjectKey::Decoy].clone_weak(),
                    transform: Transform::from_translation(event.position),
                    ..Default::default()
                },
                StateScoped(Screen::Playing),
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
