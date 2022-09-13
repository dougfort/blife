use bevy::prelude::*;
use bevy_life::SimulationPause;

use crate::cli;
use crate::pause;

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    args: Res<cli::Args>,
    mut commands: Commands,
    mut paused: ResMut<pause::PauseSwitch>,
) {
    if args.step && keys.just_pressed(KeyCode::Space) {
        match paused.0 {
            pause::PauseState::Paused => {
                commands.remove_resource::<SimulationPause>();
                paused.0 = pause::PauseState::Unpaused;
            }
            pause::PauseState::WaitFrame => (),
            pause::PauseState::Unpaused => (),
        }
    }
}

