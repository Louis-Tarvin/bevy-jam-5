use avian3d::collision::{Collider, CollisionLayers, LayerMask};
use bevy::prelude::*;
use bevy_health_bar3d::configuration::{
    BarHeight, BarSettings, ColorScheme, ForegroundColor, Percentage,
};
use rand::Rng;

use crate::{
    game::{
        assets::{HandleMap, ObjectKey},
        collision::CollisionLayer,
        gameplay::GameplayManager,
        util::Spin,
        waypoint::Waypointed,
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_asteroid);
    app.observe(spawn_random_asteroid);
    app.insert_resource(
        ColorScheme::<Asteroid>::new().foreground_color(ForegroundColor::Static(
            bevy::color::palettes::basic::SILVER.into(),
        )),
    );
}

#[derive(Event, Debug)]
pub struct SpawnAsteroid {
    pub position: Vec3,
    pub is_visible: bool,
}

#[derive(Component, Debug, Default, Reflect)]
pub struct Asteroid {
    pub contained_resources: u32,
    max_resources: u32,
}
impl Percentage for Asteroid {
    fn value(&self) -> f32 {
        self.contained_resources as f32 / self.max_resources as f32
    }
}

fn spawn_asteroid(
    trigger: Trigger<SpawnAsteroid>,
    mut commands: Commands,
    object_handles: Res<HandleMap<ObjectKey>>,
) {
    let mut rng = rand::thread_rng();
    let mut random_rotation = Quat::IDENTITY;
    random_rotation *= Quat::from_rotation_x(f32::to_radians(rng.gen_range(0.0..360.0)));
    random_rotation *= Quat::from_rotation_y(f32::to_radians(rng.gen_range(0.0..360.0)));
    random_rotation *= Quat::from_rotation_z(f32::to_radians(rng.gen_range(0.0..360.0)));
    let transform = Transform {
        translation: trigger.event().position,
        rotation: random_rotation,
        ..Default::default()
    };
    let visibility = if trigger.event().is_visible {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
    let mut entity = commands.spawn((
        Name::new("Asteroid"),
        Asteroid {
            contained_resources: 12,
            max_resources: 12,
        },
        SceneBundle {
            scene: object_handles[&ObjectKey::Asteroid].clone_weak(),
            transform,
            visibility,
            ..Default::default()
        },
        Spin {
            rotation_axis: Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            )
            .normalize(),
            rotation_speed: 0.1,
        },
        Collider::sphere(7.0),
        CollisionLayers::new([CollisionLayer::Asteroid], LayerMask::NONE),
        BarSettings::<Asteroid> {
            width: 5.0,
            offset: 6.0,
            height: BarHeight::Static(0.5),
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));
    if trigger.event().is_visible {
        entity.insert(Waypointed::new(Color::srgb(0.7, 0.4, 0.5)));
    }
}

#[derive(Event, Debug)]
pub struct SpawnRandomAsteroid;

pub fn spawn_random_asteroid(
    _trigger: Trigger<SpawnRandomAsteroid>,
    mut commands: Commands,
    mut gameplay_manager: ResMut<GameplayManager>,
) {
    let mut rng = rand::thread_rng();
    let random_angle = rng.gen::<f32>() * std::f32::consts::PI * 2.0;
    let position = Vec3::new(
        random_angle.cos() * gameplay_manager.asteroid_spawn_distance,
        random_angle.sin() * gameplay_manager.asteroid_spawn_distance,
        -5.0,
    );
    gameplay_manager.asteroid_spawn_distance += 10.0;
    commands.trigger(SpawnAsteroid {
        position,
        is_visible: false,
    });
}
