use bevy::{ecs::component::StorageType, prelude::*, window::WindowResized};

use crate::{screen::Screen, AppSet};

use super::assets::{HandleMap, ImageKey};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_waypoint);
    app.init_resource::<ScreenDimensions>();
    app.add_systems(
        Update,
        update_waypoints
            .run_if(in_state(Screen::Playing))
            .in_set(AppSet::PostUpdate),
    );
    app.add_systems(Update, on_resize_system);
}

#[derive(Component, Debug)]
pub struct UiWaypoint(pub Entity);

pub struct Waypointed {
    color: Color,
}
impl Waypointed {
    pub fn new(color: impl Into<Color>) -> Self {
        Self {
            color: color.into(),
        }
    }
}
impl Component for Waypointed {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
            let color = world.get::<Waypointed>(entity).unwrap().color;
            world.trigger::<SpawnWaypoint>(SpawnWaypoint { entity, color });
        });
    }
}

#[derive(Event, Debug)]
pub struct SpawnWaypoint {
    pub entity: Entity,
    pub color: Color,
}

fn spawn_waypoint(
    trigger: Trigger<SpawnWaypoint>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    let entity = trigger.event().entity;
    commands.spawn((
        Name::new("Waypoint"),
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..Default::default()
            },
            image: UiImage {
                texture: image_handles.get(&ImageKey::Waypoint).unwrap().clone(),
                color: trigger.event().color,
                ..Default::default()
            },
            ..Default::default()
        },
        UiWaypoint(entity),
        StateScoped(Screen::Playing),
    ));
}

#[derive(Resource, Default)]
pub struct ScreenDimensions {
    pub width: f32,
    pub height: f32,
}

fn update_waypoints(
    mut waypoint_query: Query<(Entity, &mut Style, &UiWaypoint)>,
    transform_query: Query<&Transform>,
    camera: Query<(&Camera, &GlobalTransform)>,
    screen_dimensions: Res<ScreenDimensions>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera.single();
    for (entity, mut style, ui_waypoint) in waypoint_query.iter_mut() {
        if let Ok(transform) = transform_query.get(ui_waypoint.0) {
            if let Some(ndc_pos) = camera.world_to_ndc(camera_transform, transform.translation) {
                let clamped_x = ndc_pos.x.clamp(-0.9, 0.9);
                let clamped_y = -(ndc_pos.y.clamp(-0.9, 0.9));
                let screen_x = (clamped_x + 1.0) * 0.5 * screen_dimensions.width;
                let screen_y = (clamped_y + 1.0) * 0.5 * screen_dimensions.height;
                style.left = Val::Px(screen_x - 16.0);
                style.top = Val::Px(screen_y - 16.0);
            }
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn on_resize_system(
    mut resize_reader: EventReader<WindowResized>,
    mut screen_dimensions: ResMut<ScreenDimensions>,
) {
    for e in resize_reader.read() {
        screen_dimensions.width = e.width;
        screen_dimensions.height = e.height;
    }
}
