//! Game mechanics and content.

use avian3d::PhysicsPlugins;
use bevy::prelude::*;

pub mod assets;
pub mod audio;
pub mod collision;
mod interact;
mod movement;
pub mod spawn;
pub mod state;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default(),
        audio::plugin,
        assets::plugin,
        movement::plugin,
        state::plugin,
        interact::plugin,
        spawn::plugin,
    ));
}
