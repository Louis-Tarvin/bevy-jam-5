//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    commands.trigger(SpawnPlayer);

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
