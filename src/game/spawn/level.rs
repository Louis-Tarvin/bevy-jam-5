//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use super::{
    asteroid::{SpawnAsteroid, SpawnRandomAsteroid},
    player::{SpawnCombatShip, SpawnMiningShip},
    station::SpawnStation,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    commands.trigger(SpawnStation);

    commands.trigger(SpawnCombatShip);
    commands.trigger(SpawnMiningShip);

    commands.trigger(SpawnAsteroid {
        position: Vec3::new(-40.0, -15.0, -5.0),
        is_visible: true,
    });
    commands.trigger(SpawnAsteroid {
        position: Vec3::new(60.0, -50.0, -5.0),
        is_visible: true,
    });
    commands.trigger(SpawnRandomAsteroid);
    commands.trigger(SpawnRandomAsteroid);
    commands.trigger(SpawnRandomAsteroid);
    commands.trigger(SpawnRandomAsteroid);

    commands.spawn((
        Name::new("Directional light"),
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: false,
                ..Default::default()
            },
            transform: Transform::from_rotation(Quat::from_rotation_x(-1.0)),
            ..Default::default()
        },
    ));
}
