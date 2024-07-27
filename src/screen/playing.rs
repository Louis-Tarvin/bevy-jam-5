//! The screen state for the main game loop.

use bevy::prelude::*;

use super::Screen;
use crate::game::{
    assets::{HandleMap, ImageKey, SoundtrackKey},
    audio::soundtrack::PlaySoundtrack,
    gameplay::{GameplayManager, Resources},
    notifications::Notification,
    spawn::level::SpawnLevel,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    app.insert_resource(ClearColor(Color::BLACK));
    app.insert_resource(HoldToExitTimer(0.0));

    // TODO: don't exit on escape
    app.add_systems(
        Update,
        return_to_title_screen.run_if(in_state(Screen::Playing)),
    );
}

fn enter_playing(
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut gameplay_manager: ResMut<GameplayManager>,
    mut resources: ResMut<Resources>,
) {
    commands.trigger(SpawnLevel);
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));

    crate::game::ui::draw_ui(commands, image_handles);
    gameplay_manager.reset();
    resources.reset();
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}

#[derive(Resource, Debug)]
pub struct HoldToExitTimer(f32);

fn return_to_title_screen(
    input: Res<ButtonInput<KeyCode>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut timer: ResMut<HoldToExitTimer>,
    mut notification_writer: EventWriter<Notification>,
    time: Res<Time>,
) {
    if input.just_pressed(KeyCode::Escape) {
        notification_writer.send(Notification("Hold ESC for 5s to exit".to_string()));
    }
    if input.pressed(KeyCode::Escape) {
        timer.0 += time.delta_seconds();
        if timer.0 > 5.0 {
            next_screen.set(Screen::Title);
        }
    } else {
        timer.0 = 0.0;
    }
}
