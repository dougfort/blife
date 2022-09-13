use bevy::prelude::*;
use bevy_life::SimulationPause;

pub enum PauseState {
    Paused,
    WaitFrame,
    Unpaused,
}

pub struct PauseSwitch(pub PauseState);

pub fn toggle_pause(mut commands: Commands, mut paused: ResMut<PauseSwitch>) {
    match paused.0 {
        PauseState::Paused => (),
        PauseState::WaitFrame => {
            commands.insert_resource(SimulationPause);
            paused.0 = PauseState::Paused
        }
        PauseState::Unpaused => {
            paused.0 = PauseState::WaitFrame;
        }
    }
}
