use clap::{Parser, Subcommand};
use rayon::prelude::*;
use std::path::Path;
use frame_view_tools_lib::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    /// Path to FrameView CSV file
    #[clap(short = 'i', long = "in", value_name = "PATH", value_hint = clap::ValueHint::DirPath)]
    in_file: std::path::PathBuf,

    #[clap(subcommand)]
    command: Commands,
}

// TODO: Average frame time
// TODO: Average temp(s)
// TODO: Overview command (A nice print out of the averages and stats)
#[derive(Subcommand, Debug)]
enum Commands {
    /// Outputs the average FPS
    AverageFPS,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::AverageFPS => average_fps(args.in_file.as_path()),
    }
}

fn average_fps(path: &Path) {
    let entry_vec: Vec<FrameViewCSVEntry> = deserialize_csv_into_vec(path);

    let mut vec_of_frame_times = Vec::new();

    for entry in entry_vec {
        match entry.ms_between_presents {
            None => {}
            Some(x) => {
                vec_of_frame_times.push(x);
            }
        }
    }

    let vec_of_fps: Vec<f64> = vec_of_frame_times.par_iter().map(|x| 1000f64 / x).collect();

    let average: f64 = vec_of_fps.par_iter().sum::<f64>() / vec_of_fps.len() as f64;

    println!("{:?}", average)
}


