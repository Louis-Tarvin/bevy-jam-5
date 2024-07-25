//! Game mechanics and content.

use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_health_bar3d::plugin::HealthBarPlugin;

use self::spawn::building::Destructable;

pub mod assets;
pub mod audio;
pub mod collision;
mod combat;
pub mod gameplay;
mod interact;
mod movement;
pub mod phase;
pub mod spawn;
pub mod util;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default(),
        HealthBarPlugin::<Destructable>::default(),
        audio::plugin,
        assets::plugin,
        movement::plugin,
        phase::plugin,
        interact::plugin,
        spawn::plugin,
        gameplay::plugin,
        combat::plugin,
        collision::plugin,
        util::plugin,
    ));
}
