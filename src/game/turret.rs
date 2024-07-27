use avian3d::collision::CollidingEntities;
use bevy::prelude::*;

use crate::{screen::Screen, AppSet};

use super::{combat::ShootEvent, spawn::enemy::Enemy};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, tick_timer.in_set(AppSet::TickTimers));
    app.add_systems(
        Update,
        shoot
            .run_if(in_state(Screen::Playing))
            .in_set(AppSet::Update),
    );
}

#[derive(Component, Debug)]
pub struct Turret {
    shoot_timer: Timer,
}
impl Turret {
    pub fn new(shoot_interval: f32) -> Self {
        Self {
            shoot_timer: Timer::from_seconds(shoot_interval, TimerMode::Repeating),
        }
    }
}

fn tick_timer(mut query: Query<&mut Turret>, time: Res<Time>) {
    for mut turret in query.iter_mut() {
        turret.shoot_timer.tick(time.delta());
    }
}

fn shoot(
    mut commands: Commands,
    turret_query: Query<(&Turret, &Transform, &CollidingEntities)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Turret>)>,
) {
    for (turret, turret_transform, colliding_entities) in turret_query.iter() {
        if turret.shoot_timer.just_finished() {
            if let Some(enemy_entity) = colliding_entities.iter().next() {
                if let Ok(enemy_transform) = enemy_query.get(*enemy_entity) {
                    let direction = enemy_transform.translation - turret_transform.translation;
                    commands.trigger(ShootEvent {
                        position: turret_transform.translation,
                        direction,
                    })
                }
            }
        }
    }
}
