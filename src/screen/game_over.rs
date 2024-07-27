use bevy::prelude::*;

use super::Screen;
use crate::{
    game::{audio::soundtrack::PlaySoundtrack, gameplay::GameplayManager, phase::PHASE_DURATION},
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::GameOver), enter_game_over);
    app.add_systems(OnExit(Screen::GameOver), exit_game_over);

    app.add_systems(
        Update,
        handle_game_over_action.run_if(in_state(Screen::GameOver)),
    );
    app.register_type::<GameOverAction>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum GameOverAction {
    Back,
}

fn enter_game_over(mut commands: Commands, gameplay_manager: Res<GameplayManager>) {
    let cycles_survived = gameplay_manager.elapsed_time / (PHASE_DURATION * 3.0);
    commands
        .ui_root()
        .insert(StateScoped(Screen::GameOver))
        .with_children(|children| {
            children.header("Base was destroyed");
            children.label("Game Over");
            children.label(format!("Cycles survived: {:.1}", cycles_survived));

            children.button("Main Menu").insert(GameOverAction::Back);
        });
}

fn exit_game_over(mut commands: Commands) {
    commands.trigger(PlaySoundtrack::Disable);
}

fn handle_game_over_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&GameOverAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                GameOverAction::Back => next_screen.set(Screen::Title),
            }
        }
    }
}
