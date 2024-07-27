use bevy::prelude::*;

use crate::{screen::Screen, AppSet};

use super::{
    camera::CameraTarget,
    gameplay::GameplayManager,
    movement::MovementController,
    spawn::{
        player::{CombatShip, CombatShipCameraTarget, MiningShipCameraTarget},
        station::StationCameraTarget,
    },
    ui::{BuildUi, CombatUi, GatherUi},
};

pub const PHASE_DURATION: f32 = 24.0;

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<GamePhase>();
    app.add_systems(OnEnter(GamePhase::Gather), on_gather);
    app.add_systems(OnExit(GamePhase::Gather), exit_gather);
    app.add_systems(OnEnter(GamePhase::Combat), on_combat);
    app.add_systems(OnExit(GamePhase::Combat), exit_combat);
    app.add_systems(OnEnter(GamePhase::Build), on_build);
    app.add_systems(OnExit(GamePhase::Build), exit_build);
    app.add_systems(
        Update,
        update_phase
            .run_if(in_state(Screen::Playing))
            .in_set(AppSet::PostUpdate),
    );
}

#[derive(SubStates, Clone, Eq, PartialEq, Debug, Hash, Default)]
#[source(Screen = Screen::Playing)]
pub enum GamePhase {
    Gather,
    Combat,
    #[default]
    Build,
}

fn on_gather(
    mut camera_target: ResMut<CameraTarget>,
    camera_target_query: Query<Entity, With<MiningShipCameraTarget>>,
    mut controller_query: Query<(&mut MovementController, Option<&CombatShip>)>,
    mut ui_query: Query<&mut Visibility, With<GatherUi>>,
) {
    if let Some(target) = camera_target_query.iter().next() {
        camera_target.0 = Some(target);
    }

    for (mut controller, ship) in &mut controller_query.iter_mut() {
        controller.enabled = ship.is_none();
    }

    for mut visibility in &mut ui_query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn exit_gather(mut ui_query: Query<&mut Visibility, With<GatherUi>>) {
    for mut visibility in &mut ui_query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn on_combat(
    mut camera_target: ResMut<CameraTarget>,
    camera_target_query: Query<Entity, With<CombatShipCameraTarget>>,
    mut controller_query: Query<(&mut MovementController, Option<&CombatShip>)>,
    mut ui_query: Query<&mut Visibility, With<CombatUi>>,
) {
    if let Some(target) = camera_target_query.iter().next() {
        camera_target.0 = Some(target);
    }

    for (mut controller, ship) in &mut controller_query.iter_mut() {
        controller.enabled = ship.is_some();
    }

    for mut visibility in &mut ui_query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn exit_combat(mut ui_query: Query<&mut Visibility, With<CombatUi>>) {
    for mut visibility in &mut ui_query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn on_build(
    mut camera_target: ResMut<CameraTarget>,
    camera_target_query: Query<Entity, With<StationCameraTarget>>,
    mut controller_query: Query<&mut MovementController>,
    mut ui_query: Query<&mut Visibility, With<BuildUi>>,
) {
    if let Some(target) = camera_target_query.iter().next() {
        camera_target.0 = Some(target);
    }

    for mut controller in &mut controller_query.iter_mut() {
        controller.enabled = false;
    }

    for mut visibility in &mut ui_query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn exit_build(mut ui_query: Query<&mut Visibility, With<BuildUi>>) {
    for mut visibility in &mut ui_query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn update_phase(
    current_state: Res<State<GamePhase>>,
    mut next_state: ResMut<NextState<GamePhase>>,
    mut manager: ResMut<GameplayManager>,
) {
    if manager.current_phase_time > PHASE_DURATION {
        info!(
            "update_phase called with current_phase_time: {}",
            manager.current_phase_time
        );
        match current_state.get() {
            GamePhase::Gather => next_state.set(GamePhase::Combat),
            GamePhase::Build => next_state.set(GamePhase::Gather),
            GamePhase::Combat => {
                // A full cycle has passed. Increase the difficulty
                manager.enemy_spawn_rate_multiplier += 0.4;
                manager.enemy_damage_multiplier += 0.1;
                next_state.set(GamePhase::Build)
            }
        }
        manager.current_phase_time = (manager.elapsed_time + 0.01) % PHASE_DURATION;
    }
}
