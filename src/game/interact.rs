use avian3d::spatial_query::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;

use crate::AppSet;

use super::{
    collision::CollisionLayer,
    gameplay::Resources,
    phase::GamePhase,
    spawn::building::{BuildingType, SpawnBuilding},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionController>();
    app.add_systems(
        Update,
        record_interaction_controller.in_set(AppSet::RecordInput),
    );
    app.add_systems(
        Update,
        mine.run_if(in_state(GamePhase::Gather))
            .in_set(AppSet::Update),
    );
    app.add_systems(
        Update,
        build
            .run_if(in_state(GamePhase::Build))
            .in_set(AppSet::Update),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct InteractionController {
    pub interacting: bool,
    pub just_interacted: bool,
    mining_speed_multiplier: f32,
    timer: Timer,
}
impl InteractionController {
    pub fn new(mining_time: f32) -> Self {
        Self {
            interacting: false,
            just_interacted: false,
            mining_speed_multiplier: 1.0,
            timer: Timer::from_seconds(mining_time, TimerMode::Repeating),
        }
    }
}

fn record_interaction_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut InteractionController>,
) {
    for mut controller in controller_query.iter_mut() {
        controller.interacting = input.pressed(KeyCode::KeyE) || input.pressed(KeyCode::Space);
        controller.just_interacted =
            input.just_pressed(KeyCode::KeyE) || input.just_pressed(KeyCode::Space);
    }
}

fn mine(
    mut query: Query<(&Transform, &mut InteractionController)>,
    time: Res<Time>,
    mut resources: ResMut<Resources>,
    spatial_query: SpatialQuery,
) {
    for (transform, mut controller) in query.iter_mut() {
        if controller.interacting {
            if let Some(_hit) = spatial_query.cast_ray(
                transform.translation,
                Dir3::NEG_Z,
                100.0,
                false,
                SpatialQueryFilter::from_mask(CollisionLayer::Asteroid),
            ) {
                let speed_multiplier = controller.mining_speed_multiplier;
                controller
                    .timer
                    .tick(time.delta().mul_f32(speed_multiplier));
                if controller.timer.finished() {
                    resources.0 += 1;
                    info!("Mined 1 resource. Total: {}", resources.0);
                }
            }
        }
    }
}

fn build(
    query: Query<(&Transform, &InteractionController)>,
    mut resources: ResMut<Resources>,
    mut commands: Commands,
) {
    for (transform, controller) in query.iter() {
        if controller.just_interacted && resources.0 >= 1 {
            resources.0 -= 1;
            commands.trigger(SpawnBuilding {
                building_type: BuildingType::Decoy,
                position: Vec3::new(transform.translation.x, transform.translation.y, -3.0),
            });
        }
    }
}
