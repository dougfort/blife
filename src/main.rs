use bevy::prelude::*;
use bevy_life::{ConwayCellState, GameOfLife2dPlugin, MooreCell2d, SimulationBatch};
use iyes_loopless::prelude::*;
use rand::Rng;
use std::collections::HashSet;

mod cli;
mod keyboard;
mod pattern_file;
mod pause;

use pause::toggle_pause;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Game Of Life".to_string(),
            width: 1300.,
            height: 800.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<cli::Args>()
        .add_plugin(GameOfLife2dPlugin::default())
        .insert_resource(SimulationBatch::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_map)
        .add_system(toggle_pause.run_if(in_step_mode))
        .add_system(keyboard::keyboard_input.run_if(in_step_mode))
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_map(mut commands: Commands, args: Res<cli::Args>) {
    spawn_map(&mut commands, &*args);
}

fn spawn_map(commands: &mut Commands, args: &cli::Args) {
    let (size_x, size_y) = (300, 200);
    let sprite_size = 4.;
    let color = Color::rgba(0., 0., 0., 0.);
    let live_cells = if args.pattern_file.is_empty() {
        random_map(size_x, size_y)
    } else {
        pattern_file::load_pattern(&args.pattern_file).expect("Unable to load pattern")
    };

    commands
        .spawn_bundle(SpatialBundle::from_transform(Transform::from_xyz(
            -(size_x as f32 * sprite_size) / 2.,
            -(size_y as f32 * sprite_size) / 2.,
            0.,
        )))
        .with_children(|builder| {
            for y in 0..=size_y {
                for x in 0..=size_x {
                    let state = ConwayCellState(live_cells.contains(&(x, y)));
                    builder
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(sprite_size)),
                                color,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(
                                sprite_size * x as f32,
                                sprite_size * y as f32,
                                0.,
                            ),
                            ..Default::default()
                        })
                        .insert(MooreCell2d::new(IVec2::new(x, y)))
                        .insert(state);
                }
            }
        });

    println!("map generated");
}

fn random_map(size_x: i32, size_y: i32) -> HashSet<(i32, i32)> {
    let mut rng = rand::thread_rng();
    let mut live_cells: HashSet<(i32, i32)> = HashSet::new();

    for y in 0..=size_y {
        for x in 0..=size_x {
            if rng.gen_bool(1. / 3.) {
                live_cells.insert((x, y));
            }
        }
    }

    live_cells
}

fn in_step_mode(args: Res<cli::Args>) -> bool {
    args.step
}
