use bevy::prelude::*;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Resources::default());
    app.add_sub_state::<GamePhase>();
}

#[derive(Resource, Default, Reflect, Debug)]
#[reflect(Resource)]
pub struct Resources(pub u32);

#[derive(SubStates, Clone, Eq, PartialEq, Debug, Hash, Default)]
#[source(Screen = Screen::Playing)]
pub enum GamePhase {
    #[default]
    Gather,
    Combat,
    Build,
    Research,
}
