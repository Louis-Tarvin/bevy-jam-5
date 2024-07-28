use std::time::Duration;

use avian3d::collision::contact_reporting::Collision;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::AppSet;

use super::{
    assets::SfxKey,
    audio::sfx::PlaySfx,
    phase::GamePhase,
    spawn::{bullet::Bullet, player::CombatShipTurret},
    upgrades::Upgrades,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CombatController>();
    app.add_systems(Update, tick_attack_timer.in_set(AppSet::TickTimers));
    app.add_systems(
        Update,
        record_combat_controller
            .run_if(in_state(GamePhase::Combat))
            .in_set(AppSet::RecordInput),
    );
    app.add_systems(
        Update,
        (rotate_towards_mouse, shoot)
            .chain()
            .run_if(in_state(GamePhase::Combat))
            .in_set(AppSet::Update),
    );
    app.add_systems(
        Update,
        handle_enemy_bullet_collision.in_set(AppSet::PostUpdate),
    );
    app.add_systems(
        Update,
        update_attack_time.run_if(resource_changed::<Upgrades>),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct CombatController {
    attack_time: f32,
    attack_timer: Timer,
    rotation_speed: f32,
    mouse_world_pos: Vec2,
    shoot: bool,
}
impl CombatController {
    pub fn new(attack_time: f32, rotation_speed: f32) -> Self {
        Self {
            attack_time,
            attack_timer: Timer::from_seconds(attack_time, TimerMode::Once),
            rotation_speed,
            mouse_world_pos: Vec2::ZERO,
            shoot: false,
        }
    }
}

fn tick_attack_timer(mut controller_query: Query<&mut CombatController>, time: Res<Time>) {
    for mut controller in &mut controller_query {
        controller.attack_timer.tick(time.delta());
    }
}

fn record_combat_controller(
    input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut controller_query: Query<&mut CombatController>,
) {
    // Record mouse world position.
    let (camera, camera_transform) = camera.single();
    if let Ok(window) = window.get_single() {
        if let Some(ray) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        {
            if let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Z)) {
                let world_position = ray.get_point(distance);
                for mut controller in &mut controller_query {
                    controller.mouse_world_pos = world_position.truncate();
                }
            }
        }
    }

    // Record attack input.
    for mut controller in &mut controller_query {
        controller.shoot = input.pressed(MouseButton::Left);
    }
}

fn rotate_towards_mouse(
    ship_query: Query<(&GlobalTransform, &CombatController)>,
    mut turret_query: Query<&mut Transform, With<CombatShipTurret>>,
    time: Res<Time>,
) {
    for (global_transform, controller) in ship_query.iter() {
        let mut transform = turret_query
            .get_single_mut()
            .expect("Expected single turret");
        let target_direction = (controller.mouse_world_pos
            - global_transform.translation().truncate())
        .normalize_or_zero()
        .extend(0.0);
        let current_direction = transform.rotation * Vec3::Y;
        let rotation = current_direction.angle_between(target_direction);
        if rotation > 0.0 {
            let rotation_direction = current_direction.cross(target_direction).z;
            let rotation_direction = if rotation_direction > 0.0 { 1.0 } else { -1.0 };
            transform.rotate(Quat::from_rotation_z(
                rotation_direction * controller.rotation_speed * rotation * time.delta_seconds(),
            ));
        }
    }
}

#[derive(Event, Debug)]
pub struct ShootEvent {
    pub position: Vec3,
    pub direction: Vec3,
}

fn shoot(
    mut ship_query: Query<&mut CombatController>,
    turret_query: Query<(&GlobalTransform, &Transform), With<CombatShipTurret>>,
    mut commands: Commands,
) {
    for mut controller in ship_query.iter_mut() {
        let attack_time = controller.attack_time;
        if controller.shoot && controller.attack_timer.finished() {
            controller
                .attack_timer
                .set_duration(Duration::from_secs_f32(attack_time));
            controller.attack_timer.reset();
            for (global_transform, transform) in turret_query.iter() {
                let mut position = global_transform.translation();
                position.z = -3.0;
                let direction = transform.rotation * Vec3::Y;
                commands.trigger(ShootEvent {
                    position,
                    direction,
                });
                commands.trigger(PlaySfx::Key(SfxKey::Shoot));
            }
        }
    }
}

fn handle_enemy_bullet_collision(
    mut collision_event_reader: EventReader<Collision>,
    bullets: Query<Entity, With<Bullet>>,
    mut commands: Commands,
) {
    for Collision(contacts) in collision_event_reader.read() {
        if bullets.contains(contacts.entity1) || bullets.contains(contacts.entity2) {
            commands.entity(contacts.entity1).despawn_recursive();
            commands.entity(contacts.entity2).despawn_recursive();
        }
    }
}

fn update_attack_time(mut controller_query: Query<&mut CombatController>, upgrades: Res<Upgrades>) {
    for mut controller in &mut controller_query {
        controller.attack_time = 1.0 / (1.0 + upgrades.fire_rate as f32);
        if upgrades.fire_rate > 0 {
            controller.attack_time += 0.2;
        }
    }
}
