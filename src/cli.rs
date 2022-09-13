use bevy::prelude::*;
use structopt::StructOpt;

use crate::pause::{PauseSwitch, PauseState};
use bevy_life::SimulationPause;

#[derive(Debug, StructOpt)]
#[structopt(name = "blife", about = "A simple CLI for Conway's Game of Life")]
pub struct Args {
    /// start in step mode
    /// (press space to advance one step)
    #[structopt(short, long)]
    pub step: bool,

    /// load a pattern file
    #[structopt(short, long, default_value = "")]
    pub pattern_file: String,
}

impl FromWorld for Args {
    fn from_world(world: &mut World) -> Self {
        let args = Args::from_args();

        if args.step {
            world.insert_resource(PauseSwitch(PauseState::Paused));
            world.insert_resource(SimulationPause);
        }

        // if args.step {
        //     world
        //         .add_system(toggle_pause)
        //         .add_system(keyboard_input);
        // }

        args
    }
}