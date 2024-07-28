//! Game mechanics and content.

use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_health_bar3d::plugin::HealthBarPlugin;

use self::{
    mining::InteractionProgressBar,
    spawn::{asteroid::Asteroid, building::Destructable},
};

pub mod assets;
pub mod audio;
pub mod build;
pub mod camera;
pub mod collision;
mod combat;
pub mod gameplay;
mod mining;
mod movement;
pub mod notifications;
pub mod phase;
pub mod spawn;
pub mod turret;
pub mod ui;
pub mod upgrades;
pub mod util;
pub mod waypoint;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default(),
        HealthBarPlugin::<Destructable>::default(),
        HealthBarPlugin::<InteractionProgressBar>::default(),
        HealthBarPlugin::<Asteroid>::default(),
        notifications::plugin,
        audio::plugin,
    ));
    app.add_plugins((
        assets::plugin,
        movement::plugin,
        phase::plugin,
        mining::plugin,
        spawn::plugin,
        gameplay::plugin,
        combat::plugin,
        collision::plugin,
        util::plugin,
        camera::plugin,
        ui::plugin,
        build::plugin,
        turret::plugin,
        waypoint::plugin,
        upgrades::plugin,
    ));
}
