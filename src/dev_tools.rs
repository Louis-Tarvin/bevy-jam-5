//! Development tools for the game. This plugin is only enabled in dev builds.

use avian3d::{collision::contact_reporting::Collision, debug_render::PhysicsDebugPlugin};
use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_toggle_active, prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{game::phase::GamePhase, screen::Screen};

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_plugins((
        WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F1)),
        PhysicsDebugPlugin::default(),
    ))
    .add_systems(Update, (log_transitions::<Screen>, print_collisions))
    .add_systems(
        Update,
        (toggle_state, log_transitions::<GamePhase>).run_if(in_state(Screen::Playing)),
    );
}

fn toggle_state(
    current_state: Res<State<GamePhase>>,
    mut next_state: ResMut<NextState<GamePhase>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F2) {
        match current_state.get() {
            GamePhase::Gather => next_state.set(GamePhase::Combat),
            GamePhase::Combat => next_state.set(GamePhase::Build),
            GamePhase::Build => next_state.set(GamePhase::Research),
            GamePhase::Research => next_state.set(GamePhase::Gather),
        }
    }
}

fn print_collisions(mut collision_event_reader: EventReader<Collision>) {
    for Collision(contacts) in collision_event_reader.read() {
        println!(
            "Entities {:?} and {:?} are colliding",
            contacts.entity1, contacts.entity2,
        );
    }
}
