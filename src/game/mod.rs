//! Game mechanics and content.

use avian3d::PhysicsPlugins;
use bevy::prelude::*;

pub mod assets;
pub mod audio;
pub mod collision;
mod combat;
pub mod gameplay;
mod interact;
mod movement;
pub mod phase;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default(),
        audio::plugin,
        assets::plugin,
        movement::plugin,
        phase::plugin,
        interact::plugin,
        spawn::plugin,
        gameplay::plugin,
        combat::plugin,
    ));
}
