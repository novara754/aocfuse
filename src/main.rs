use std::path::PathBuf;

use fuse::AoCFS;
use fuser::MountOption;

use clap::Parser;

mod fuse;
mod parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    aoc_path: PathBuf,
    mountpoint: PathBuf,
}

fn main() {
    let args = Args::parse();

    let aoc_input = std::fs::read_to_string(args.aoc_path).unwrap();

    fuser::mount2(
        AoCFS::new(&aoc_input),
        args.mountpoint,
        &[MountOption::AutoUnmount, MountOption::RO],
    )
    .unwrap();
}
