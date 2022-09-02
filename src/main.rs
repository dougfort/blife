use bevy::prelude::*;
use bevy_life::{
    ConwayCellState, GameOfLife2dPlugin, MooreCell2d, SimulationBatch, SimulationPause,
};
use rand::Rng;
use std::collections::HashSet;

mod pattern_file;

enum PauseState {
    Paused,
    WaitFrame,
    Unpaused,
}
struct PauseSwitch(PauseState);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Game Of Life".to_string(),
            width: 1300.,
            height: 800.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GameOfLife2dPlugin::default())
        .insert_resource(SimulationBatch::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_map)
        .add_system(toggle_pause)
        .add_system(keyboard_input)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_map(mut commands: Commands) {
    spawn_map(&mut commands);
}

fn spawn_map(commands: &mut Commands) {
    let (size_x, size_y) = (300, 200);
    let sprite_size = 4.;
    let color = Color::rgba(0., 0., 0., 0.);
    //    let live_cells = random_map(size_x, size_y);
//    let live_cells = blinker_map(size_x, size_y);
    let live_cells = pattern_file::load_pattern("patterns/glider.cells").expect("Unable to load pattern");

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
    commands.insert_resource(PauseSwitch(PauseState::Paused));
    commands.insert_resource(SimulationPause);

    println!("map generated");
}

fn toggle_pause(mut commands: Commands, mut paused: ResMut<PauseSwitch>) {
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

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut paused: ResMut<PauseSwitch>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match paused.0 {
            PauseState::Paused => {
                commands.remove_resource::<SimulationPause>();
                paused.0 = PauseState::Unpaused;
            }
            PauseState::WaitFrame => (),
            PauseState::Unpaused => (),
        }
    }
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

fn zero_zero_map(_size_x: i32, _size_y: i32) -> HashSet<(i32, i32)> {
    let mut live_cells: HashSet<(i32, i32)> = HashSet::new();
    live_cells.insert((0, 0));
    live_cells
}

fn blinker_map(_size_x: i32, _size_y: i32) -> HashSet<(i32, i32)> {
    let mut live_cells: HashSet<(i32, i32)> = HashSet::new();
    live_cells.insert((3, 0));
    live_cells.insert((3, 1));
    live_cells.insert((3, 2));
    live_cells
}
