//! Development tools for the game. This plugin is only enabled in dev builds.

use avian3d::debug_render::PhysicsDebugPlugin;
use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_toggle_active, prelude::*,
};
#[cfg(feature = "bevy-inspector-egui")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_plugins((
        #[cfg(feature = "bevy-inspector-egui")]
        WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F1)),
        PhysicsDebugPlugin::default(),
    ))
    .add_systems(Update, log_transitions::<Screen>);
}
