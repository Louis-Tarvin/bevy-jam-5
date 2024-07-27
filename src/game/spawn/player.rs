//! Spawn the player.

use bevy::prelude::*;
use bevy_health_bar3d::configuration::{BarHeight, BarSettings};

use crate::{
    game::{
        assets::{HandleMap, ObjectKey},
        combat::CombatController,
        interact::{InteractionController, InteractionProgressBar},
        movement::{MovementController, Velocity},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_combat_ship);
    app.observe(spawn_mining_ship);
}

#[derive(Event, Debug)]
pub struct SpawnCombatShip;

#[derive(Component)]
pub struct CombatShip;

#[derive(Component)]
pub struct CombatShipTurret;

#[derive(Component)]
pub struct CombatShipCameraTarget;

fn spawn_combat_ship(
    _trigger: Trigger<SpawnCombatShip>,
    mut commands: Commands,
    object_handles: Res<HandleMap<ObjectKey>>,
) {
    commands
        .spawn((
            Name::new("CombatShip"),
            SceneBundle {
                scene: object_handles[&ObjectKey::ShipBody].clone_weak(),
                transform: Transform::from_xyz(10.0, -10.0, 0.0),
                ..Default::default()
            },
            CombatShip,
            MovementController::new(50.0, 0.5, 100.0),
            CombatController::new(1.0, 5.0),
            Velocity::default(),
            StateScoped(Screen::Playing),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Turret"),
                SceneBundle {
                    scene: object_handles[&ObjectKey::ShipTurret].clone_weak(),
                    ..Default::default()
                },
                CombatShipTurret,
            ));

            parent.spawn((
                Name::new("CameraTarget"),
                Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
                GlobalTransform::default(),
                CombatShipCameraTarget,
            ));
        });
}

#[derive(Event, Debug)]
pub struct SpawnMiningShip;

#[derive(Component)]
pub struct MiningShip;

#[derive(Component)]
pub struct MiningShipCameraTarget;

fn spawn_mining_ship(
    _trigger: Trigger<SpawnMiningShip>,
    mut commands: Commands,
    object_handles: Res<HandleMap<ObjectKey>>,
) {
    commands
        .spawn((
            Name::new("MiningShip"),
            SceneBundle {
                scene: object_handles[&ObjectKey::ShipBody].clone_weak(),
                transform: Transform::from_xyz(-10.0, -10.0, 0.0),
                ..Default::default()
            },
            MiningShip,
            MovementController::new(40.0, 0.6, 800.0),
            InteractionController::new(1.0),
            Velocity::default(),
            StateScoped(Screen::Playing),
            InteractionProgressBar::default(),
            BarSettings::<InteractionProgressBar> {
                width: 5.0,
                offset: 3.0,
                height: BarHeight::Static(0.5),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("CameraTarget"),
                Transform::from_translation(Vec3::new(0.0, 0.0, 150.0)),
                GlobalTransform::default(),
                MiningShipCameraTarget,
            ));
        });
}
