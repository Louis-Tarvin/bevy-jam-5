use bevy::prelude::*;

use crate::{screen::Screen, AppSet};

use super::spawn::enemy::SpawnEnemy;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Resources::default());
    app.insert_resource(GameplayManager::default());
    app.add_systems(
        Update,
        tick_time
            .run_if(in_state(Screen::Playing))
            .in_set(AppSet::TickTimers),
    );
    app.add_systems(
        Update,
        spawn_enemies
            .run_if(in_state(Screen::Playing))
            .in_set(AppSet::Update),
    );
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct Resources {
    pub gathered: u32,
    pub delivered: u32,
}
impl Default for Resources {
    fn default() -> Self {
        Self {
            gathered: 0,
            delivered: 15,
        }
    }
}
impl Resources {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct GameplayManager {
    enemy_spawn_timer: Timer,
    peace_timer: Timer,
    pub enemy_spawn_rate_multiplier: f32,
    pub elapsed_time: f32,
    pub current_phase_time: f32,
    pub asteroid_spawn_distance: f32,
}
impl Default for GameplayManager {
    fn default() -> Self {
        Self {
            enemy_spawn_timer: Timer::from_seconds(6.0, TimerMode::Repeating),
            peace_timer: Timer::from_seconds(30.0, TimerMode::Once),
            enemy_spawn_rate_multiplier: 1.0,
            elapsed_time: 0.0,
            current_phase_time: 0.0,
            asteroid_spawn_distance: 100.0,
        }
    }
}
impl GameplayManager {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

const ENEMY_SPAWN_DISTANCE: f32 = 100.0;

fn tick_time(mut manager: ResMut<GameplayManager>, time: Res<Time>) {
    let spawn_rate_multiplier = manager.enemy_spawn_rate_multiplier;
    manager.elapsed_time += time.delta_seconds();
    manager.current_phase_time += time.delta_seconds();
    manager
        .enemy_spawn_timer
        .tick(time.delta().mul_f32(spawn_rate_multiplier));
    manager.peace_timer.tick(time.delta());
}

fn spawn_enemies(mut commands: Commands, manager: Res<GameplayManager>) {
    if manager.enemy_spawn_timer.just_finished() && manager.peace_timer.finished() {
        let random_angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
        let position = Vec3::new(
            random_angle.cos() * ENEMY_SPAWN_DISTANCE,
            random_angle.sin() * ENEMY_SPAWN_DISTANCE,
            -3.0,
        );
        commands.trigger(SpawnEnemy { position });
    }
}
