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
            delivered: 10,
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
    enemy_spawn_rate_multiplier: f32,
    pub enemy_damage_multiplier: f32,
    pub elapsed_time: f32,
    pub current_phase_time: f32,
    pub asteroid_spawn_distance: f32,
    pub cycle: u32,
}
impl Default for GameplayManager {
    fn default() -> Self {
        Self {
            enemy_spawn_timer: Timer::from_seconds(5.0, TimerMode::Repeating),
            peace_timer: Timer::from_seconds(30.0, TimerMode::Once),
            enemy_spawn_rate_multiplier: 1.0,
            enemy_damage_multiplier: 1.0,
            elapsed_time: 0.0,
            current_phase_time: 0.0,
            asteroid_spawn_distance: 100.0,
            cycle: 0,
        }
    }
}
impl GameplayManager {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn new_cycle(&mut self) {
        // Increase difficulty
        self.enemy_spawn_rate_multiplier += 0.2;
        self.enemy_damage_multiplier += 0.1;
        self.peace_timer.reset();
        self.cycle += 1;
    }

    pub fn stop_spawning(&mut self) {
        self.peace_timer.reset();
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
        commands.trigger(SpawnEnemy {
            distance: ENEMY_SPAWN_DISTANCE,
            damage_mult: manager.enemy_damage_multiplier,
        });
    }
}
