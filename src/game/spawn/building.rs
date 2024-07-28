use avian3d::{
    collision::{Collider, CollisionLayers, LayerMask, Sensor},
    dynamics::rigid_body::RigidBody,
};
use bevy::prelude::*;
use bevy_health_bar3d::configuration::{BarHeight, BarSettings, Percentage};

use crate::{
    game::{
        assets::{HandleMap, ObjectKey, SfxKey},
        audio::sfx::PlaySfx,
        collision::CollisionLayer,
        notifications::Notification,
        turret::Turret,
        upgrades::{Upgrade, UpgradeType},
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
    Sniper,
    MiningUpgrade,
    FireRateUpgrade,
}
impl BuildingType {
    pub fn cost(&self) -> u32 {
        match self {
            BuildingType::Decoy => 3,
            BuildingType::Turret => 12,
            BuildingType::Sniper => 15,
            BuildingType::FireRateUpgrade => 9,
            BuildingType::MiningUpgrade => 14,
        }
    }
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let event = trigger.event();
    match event.building_type {
        BuildingType::Decoy => {
            commands
                .spawn((
                    Name::new("Decoy"),
                    Destructable::new(200.0),
                    SceneBundle {
                        scene: object_handles[&ObjectKey::Decoy].clone_weak(),
                        transform: Transform::from_translation(event.position)
                            .with_scale(Vec3::splat(2.0)),
                        ..Default::default()
                    },
                    StateScoped(Screen::Playing),
                    BarSettings::<Destructable> {
                        width: 5.0,
                        offset: 3.0,
                        height: BarHeight::Static(0.5),
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Light"),
                        PbrBundle {
                            mesh: meshes.add(Sphere { radius: 0.1 }.mesh().ico(1).unwrap()),
                            material: materials.add(Color::srgb(0.5, 0.5, 6.0)),
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.3)),
                            ..Default::default()
                        },
                    ));
                });
        }
        BuildingType::Turret => {
            commands
                .spawn((
                    Name::new("Turret"),
                    Turret::new(1.2),
                    Destructable::new(50.0),
                    SceneBundle {
                        scene: object_handles[&ObjectKey::Decoy].clone_weak(),
                        transform: Transform::from_translation(event.position)
                            .with_scale(Vec3::splat(2.0)),
                        ..Default::default()
                    },
                    StateScoped(Screen::Playing),
                    BarSettings::<Destructable> {
                        width: 5.0,
                        offset: 3.0,
                        height: BarHeight::Static(0.5),
                        ..Default::default()
                    },
                    Collider::sphere(7.0),
                    RigidBody::Static,
                    Sensor,
                    CollisionLayers::new(
                        [CollisionLayer::Turret],
                        LayerMask::from(CollisionLayer::Enemy),
                    ),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Light"),
                        PbrBundle {
                            mesh: meshes.add(Sphere { radius: 0.1 }.mesh().ico(1).unwrap()),
                            material: materials.add(Color::srgb(6.0, 0.5, 0.5)),
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.3)),
                            ..Default::default()
                        },
                    ));
                });
        }
        BuildingType::Sniper => {
            commands
                .spawn((
                    Name::new("Sniper"),
                    Turret::new(2.5),
                    Destructable::new(50.0),
                    SceneBundle {
                        scene: object_handles[&ObjectKey::Decoy].clone_weak(),
                        transform: Transform::from_translation(event.position)
                            .with_scale(Vec3::splat(2.0)),
                        ..Default::default()
                    },
                    StateScoped(Screen::Playing),
                    BarSettings::<Destructable> {
                        width: 5.0,
                        offset: 3.0,
                        height: BarHeight::Static(0.5),
                        ..Default::default()
                    },
                    Collider::sphere(20.0),
                    RigidBody::Static,
                    Sensor,
                    CollisionLayers::new(
                        [CollisionLayer::Turret],
                        LayerMask::from(CollisionLayer::Enemy),
                    ),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Light"),
                        PbrBundle {
                            mesh: meshes.add(Sphere { radius: 0.1 }.mesh().ico(1).unwrap()),
                            material: materials.add(Color::srgb(3.0, 0.5, 3.0)),
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.3)),
                            ..Default::default()
                        },
                    ));
                });
        }
        BuildingType::FireRateUpgrade => {
            commands
                .spawn((
                    Name::new("Fire-rate upgrade"),
                    Destructable::new(60.0),
                    Upgrade(UpgradeType::FireRate),
                    SceneBundle {
                        scene: object_handles[&ObjectKey::Upgrade].clone_weak(),
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
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Light"),
                        PbrBundle {
                            mesh: meshes.add(Sphere { radius: 0.2 }.mesh().ico(1).unwrap()),
                            material: materials.add(Color::srgb(0.5, 6.0, 0.5)),
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.8)),
                            ..Default::default()
                        },
                    ));
                });
        }
        BuildingType::MiningUpgrade => {
            commands
                .spawn((
                    Name::new("Mining upgrade"),
                    Destructable::new(60.0),
                    Upgrade(UpgradeType::MiningSpeed),
                    SceneBundle {
                        scene: object_handles[&ObjectKey::Upgrade].clone_weak(),
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
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Light"),
                        PbrBundle {
                            mesh: meshes.add(Sphere { radius: 0.2 }.mesh().ico(1).unwrap()),
                            material: materials.add(Color::srgb(3.0, 3.0, 0.5)),
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.8)),
                            ..Default::default()
                        },
                    ));
                });
        }
    }
}

fn destroy_building(
    mut commands: Commands,
    query: Query<(Entity, &Destructable, &Name), Changed<Destructable>>,
    mut notification_writer: ResMut<Events<Notification>>,
) {
    for (entity, destructable, name) in query.iter() {
        if destructable.health <= 0.0 {
            notification_writer.send(Notification(format!("{} destroyed", name.as_str())));
            commands.entity(entity).despawn_recursive();
            commands.trigger(PlaySfx::Key(SfxKey::Explode));
        }
    }
}
