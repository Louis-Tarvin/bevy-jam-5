//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use super::{asteroid::SpawnAsteroid, player::SpawnPlayer, station::SpawnStation};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    commands.trigger(SpawnStation);

    commands.trigger(SpawnPlayer);

    commands.trigger(SpawnAsteroid {
        position: Vec3::new(10.0, 10.0, -5.0),
    });
    commands.trigger(SpawnAsteroid {
        position: Vec3::new(-10.0, -10.0, -5.0),
    });
    commands.trigger(SpawnAsteroid {
        position: Vec3::new(10.0, -10.0, -5.0),
    });

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
