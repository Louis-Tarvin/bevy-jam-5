//! Spawn the player.

use bevy::prelude::*;

use crate::{
    game::{
        assets::{HandleMap, ObjectKey},
        combat::CombatController,
        interact::InteractionController,
        movement::{MovementController, Velocity},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerTurret;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    object_handles: Res<HandleMap<ObjectKey>>,
) {
    commands
        .spawn((
            Name::new("Player"),
            Player,
            SceneBundle {
                scene: object_handles[&ObjectKey::ShipBody].clone_weak(),
                ..Default::default()
            },
            MovementController::new(50.0, 0.5, 100.0),
            CombatController::new(1.0, 5.0),
            InteractionController::new(1.0),
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
                PlayerTurret,
            ));
        });
}
