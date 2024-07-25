use bevy::prelude::*;

use crate::screen::Screen;

use super::spawn::enemy::SpawnEnemy;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Resources(5));
    app.insert_resource(GameplayManager {
        enemy_spawn_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });
    app.add_systems(Update, spawn_enemies.run_if(in_state(Screen::Playing)));
}

#[derive(Resource, Default, Reflect, Debug)]
#[reflect(Resource)]
pub struct Resources(pub u32);

#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct GameplayManager {
    enemy_spawn_timer: Timer,
}

const ENEMY_SPAWN_DISTANCE: f32 = 100.0;

fn spawn_enemies(mut commands: Commands, time: Res<Time>, mut manager: ResMut<GameplayManager>) {
    if manager.enemy_spawn_timer.tick(time.delta()).just_finished() {
        let random_angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
        let position = Vec3::new(
            random_angle.cos() * ENEMY_SPAWN_DISTANCE,
            random_angle.sin() * ENEMY_SPAWN_DISTANCE,
            -3.0,
        );
        commands.trigger(SpawnEnemy { position });
    }
}
