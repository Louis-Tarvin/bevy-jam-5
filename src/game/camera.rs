use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::AppSet;

use super::{build::BuildLocationMarker, movement::Velocity, phase::GamePhase};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CameraTarget>();
    app.insert_resource(DustParticleTimer {
        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        cached_mesh: None,
        cached_material: None,
    });
    app.add_systems(
        Update,
        (pan_camera_target, reset_camera_target)
            .run_if(in_state(GamePhase::Build))
            .in_set(AppSet::Update),
    );
    app.add_systems(Update, update_velocity_offset.in_set(AppSet::Update));
    app.add_systems(Update, move_camera_to_target.in_set(AppSet::PostUpdate));
    app.add_systems(
        Update,
        (spawn_dust_particles, update_dust_particles)
            .chain()
            .in_set(AppSet::Update),
    );
}

#[derive(Resource, Default)]
pub struct CameraTarget(pub Option<Entity>);

fn move_camera_to_target(
    camera_target: Res<CameraTarget>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    target_query: Query<&GlobalTransform, Without<Camera>>,
) {
    if let Some(target) = camera_target.0 {
        if let Ok(target_transform) = target_query.get(target) {
            for mut camera_transform in camera_query.iter_mut() {
                camera_transform.translation = camera_transform
                    .translation
                    .lerp(target_transform.translation(), 0.1);
            }
        }
    }
}

const PAN_SENSITIVITY: f32 = 0.1;

fn pan_camera_target(
    camera_target: Res<CameraTarget>,
    mut target_query: Query<&mut Transform>,
    mut mouse_motion: EventReader<MouseMotion>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    build_marker: Query<&BuildLocationMarker>,
) {
    let mut total_motion: Vec2 = mouse_motion.read().map(|ev| ev.delta).sum();
    total_motion.y = -total_motion.y;

    // Hacky way to prevent panning when placing a building
    if let Some(marker) = build_marker.iter().next() {
        if marker.mode.is_some() {
            return;
        }
    }

    if mouse_input.pressed(MouseButton::Left) {
        if let Some(target) = camera_target.0 {
            if let Ok(mut target_transform) = target_query.get_mut(target) {
                target_transform.translation.x -= total_motion.x * PAN_SENSITIVITY;
                target_transform.translation.y -= total_motion.y * PAN_SENSITIVITY;
            }
        }
    }
}

fn reset_camera_target(
    camera_target: Res<CameraTarget>,
    mut target_query: Query<&mut Transform>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::KeyE) {
        if let Some(target) = camera_target.0 {
            if let Ok(mut target_transform) = target_query.get_mut(target) {
                target_transform.translation.x = 0.0;
                target_transform.translation.y = 0.0;
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct OffsetDistanceByVelocity(pub f32);

fn update_velocity_offset(
    mut target_query: Query<(&mut Transform, &OffsetDistanceByVelocity, &Parent)>,
    velocity_query: Query<&Velocity>,
) {
    for (mut target_transform, offset, parent) in target_query.iter_mut() {
        if let Ok(velocity) = velocity_query.get(parent.get()) {
            target_transform.translation.z = offset.0 + velocity.0.length() * 0.3;
        }
    }
}

#[derive(Debug)]
enum DustParticleState {
    FadeIn,
    FadeOut,
    Idle,
}

const DUST_PARTICLE_LIFETIME: f32 = 3.0;

#[derive(Component, Debug)]
struct DustParticle {
    state: DustParticleState,
    timer: Timer,
}

impl DustParticle {
    fn new() -> Self {
        Self {
            state: DustParticleState::FadeIn,
            timer: Timer::from_seconds(DUST_PARTICLE_LIFETIME, TimerMode::Repeating),
        }
    }
}

fn update_dust_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DustParticle, &mut Transform)>,
) {
    for (entity, mut particle, mut transform) in query.iter_mut() {
        particle.timer.tick(time.delta());
        match particle.state {
            DustParticleState::FadeIn => {
                transform.scale = Vec3::splat(particle.timer.elapsed_secs());
                if particle.timer.just_finished() {
                    transform.scale = Vec3::splat(DUST_PARTICLE_LIFETIME);
                    particle.state = DustParticleState::Idle;
                }
            }
            DustParticleState::FadeOut => {
                transform.scale =
                    Vec3::splat(DUST_PARTICLE_LIFETIME - particle.timer.elapsed_secs());
                if particle.timer.just_finished() {
                    commands.entity(entity).despawn();
                }
            }
            DustParticleState::Idle => {
                if particle.timer.just_finished() {
                    particle.state = DustParticleState::FadeOut;
                }
            }
        }
    }
}

#[derive(Resource, Debug)]
struct DustParticleTimer {
    timer: Timer,
    cached_mesh: Option<Handle<Mesh>>,
    cached_material: Option<Handle<StandardMaterial>>,
}

fn spawn_dust_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<DustParticleTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera: Query<&Transform, With<Camera>>,
) {
    timer.timer.tick(time.delta());
    if timer.timer.just_finished() {
        let material = timer
            .cached_material
            .get_or_insert_with(|| {
                materials.add(StandardMaterial {
                    base_color: Color::srgb(0.5, 0.5, 0.5),
                    unlit: true,
                    ..Default::default()
                })
            })
            .clone();
        let mesh = timer
            .cached_mesh
            .get_or_insert_with(|| meshes.add(Sphere { radius: 0.1 }.mesh().ico(1).unwrap()))
            .clone();
        let random_dx = (rand::random::<f32>() - 0.5) * 300.0;
        let random_dy = (rand::random::<f32>() - 0.5) * 300.0;
        let random_z = (rand::random::<f32>() - 0.5) * 50.0;
        let camera_transform = camera.iter().next().unwrap();
        let pos = Vec3::new(
            camera_transform.translation.x + random_dx,
            camera_transform.translation.y + random_dy,
            random_z,
        );
        commands.spawn((
            DustParticle::new(),
            PbrBundle {
                material,
                mesh,
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
        ));
    }
}
