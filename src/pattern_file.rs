use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

pub fn load_pattern(filename: &str) -> Result<HashSet<(i32, i32)>> {
    let mut live_cells: HashSet<(i32, i32)> = HashSet::new();
    let file = File::open(filename)?;
    io::BufReader::new(file)
        .lines()
        .filter(|line| {
            line.as_ref()
                .map(|line| !line.starts_with('!'))
                .unwrap_or(false)
        })
        .enumerate()
        .for_each(|(y, line)| {
            let line = line.unwrap();
            line.chars().enumerate().for_each(|(x, c)| {
                if c == 'O' {
                    live_cells.insert((x as i32, y as i32));
                }
            });
        });
    Ok(live_cells)
}
