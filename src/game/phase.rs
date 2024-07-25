use bevy::prelude::*;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<GamePhase>();
}

#[derive(SubStates, Clone, Eq, PartialEq, Debug, Hash, Default)]
#[source(Screen = Screen::Playing)]
pub enum GamePhase {
    Gather,
    Combat,
    #[default]
    Build,
    Research,
}
