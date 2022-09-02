use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "blife", about = "A simple CLI for Conway's Game of Life")]
pub struct Args {
    /// start in step mode
    /// (press space to advance one step)
    #[structopt(short, long)]
    pub step: bool,
}

pub fn parse_args() -> Args {
    Args::from_args()
}
