use bevy::prelude::*;

use crate::{screen::Screen, AppSet};

use super::spawn::enemy::SpawnEnemy;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Resources {
        gathered: 0,
        delivered: 5,
    });
    app.insert_resource(GameplayManager {
        enemy_spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        elapsed_time: 0.0,
        current_phase_time: 0.0,
        asteroid_spawn_distance: 100.0,
    });
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

#[derive(Resource, Default, Reflect, Debug)]
#[reflect(Resource)]
pub struct Resources {
    pub gathered: u32,
    pub delivered: u32,
}

#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct GameplayManager {
    enemy_spawn_timer: Timer,
    pub elapsed_time: f32,
    pub current_phase_time: f32,
    pub asteroid_spawn_distance: f32,
}

const ENEMY_SPAWN_DISTANCE: f32 = 100.0;

fn tick_time(mut manager: ResMut<GameplayManager>, time: Res<Time>) {
    manager.elapsed_time += time.delta_seconds();
    manager.current_phase_time += time.delta_seconds();
    manager.enemy_spawn_timer.tick(time.delta());
}

fn spawn_enemies(mut commands: Commands, manager: Res<GameplayManager>) {
    if manager.enemy_spawn_timer.just_finished() {
        let random_angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
        let position = Vec3::new(
            random_angle.cos() * ENEMY_SPAWN_DISTANCE,
            random_angle.sin() * ENEMY_SPAWN_DISTANCE,
            -3.0,
        );
        commands.trigger(SpawnEnemy { position });
    }
}
