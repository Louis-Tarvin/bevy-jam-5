use avian3d::spatial_query::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use bevy_health_bar3d::configuration::{ColorScheme, ForegroundColor, Percentage};

use crate::AppSet;

use super::{
    collision::CollisionLayer,
    gameplay::Resources,
    phase::GamePhase,
    spawn::asteroid::{Asteroid, SpawnRandomAsteroid},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MiningController>();
    app.insert_resource(
        ColorScheme::<InteractionProgressBar>::new().foreground_color(ForegroundColor::Static(
            bevy::color::palettes::tailwind::ORANGE_600.into(),
        )),
    );
    app.add_systems(
        Update,
        record_interaction_controller.in_set(AppSet::RecordInput),
    );
    app.add_systems(
        Update,
        (
            (mine, destroy_empty_asteroids).chain(),
            set_progress,
            deliver_resources,
        )
            .run_if(in_state(GamePhase::Gather))
            .in_set(AppSet::Update),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MiningController {
    pub interacting: bool,
    pub just_interacted: bool,
    mining_time: f32,
    mining_speed_multiplier: f32,
    timer: Timer,
}
impl MiningController {
    pub fn new(mining_time: f32) -> Self {
        Self {
            interacting: false,
            just_interacted: false,
            mining_time,
            mining_speed_multiplier: 1.0,
            timer: Timer::from_seconds(mining_time, TimerMode::Repeating),
        }
    }
}

fn record_interaction_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MiningController>,
) {
    for mut controller in controller_query.iter_mut() {
        controller.interacting = input.pressed(KeyCode::KeyE) || input.pressed(KeyCode::Space);
        controller.just_interacted =
            input.just_pressed(KeyCode::KeyE) || input.just_pressed(KeyCode::Space);
        if input.just_released(KeyCode::KeyE) || input.just_released(KeyCode::Space) {
            controller.timer.reset();
        }
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct InteractionProgressBar(pub f32);

impl Percentage for InteractionProgressBar {
    fn value(&self) -> f32 {
        self.0
    }
}

fn set_progress(mut progress_query: Query<(&MiningController, &mut InteractionProgressBar)>) {
    for (controller, mut progress) in progress_query.iter_mut() {
        if controller.interacting {
            progress.0 = controller.timer.elapsed_secs() / controller.mining_time;
        } else {
            progress.0 = 0.0;
        }
    }
}

fn mine(
    mut query: Query<(&Transform, &mut MiningController)>,
    mut asteroid_query: Query<&mut Asteroid>,
    time: Res<Time>,
    mut resources: ResMut<Resources>,
    spatial_query: SpatialQuery,
) {
    for (transform, mut controller) in query.iter_mut() {
        if controller.interacting {
            if let Some(hit) = spatial_query.cast_ray(
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
                    resources.gathered += 1;
                    if let Ok(mut asteroid) = asteroid_query.get_mut(hit.entity) {
                        asteroid.contained_resources -= 1;
                    }
                }
            }
        }
    }
}

fn deliver_resources(
    mut resources: ResMut<Resources>,
    mut query: Query<&Transform, With<MiningController>>,
) {
    // If within 15m of the station (origin), deliver resources
    for transform in query.iter_mut() {
        if transform.translation.xy().length() < 15.0 {
            resources.delivered += resources.gathered;
            resources.gathered = 0;
        }
    }
}

fn destroy_empty_asteroids(mut commands: Commands, query: Query<(Entity, &Asteroid)>) {
    for (entity, asteroid) in query.iter() {
        if asteroid.contained_resources == 0 {
            commands.entity(entity).despawn_recursive();
            commands.trigger(SpawnRandomAsteroid);
        }
    }
}
