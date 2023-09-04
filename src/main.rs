use iced::{Settings, Application};
mod window;
use window::*;

use clap::Parser;
use std::{fs, path::PathBuf};


#[derive(Parser, Debug)]
struct Args {
    image_path: String
}

fn main() {
   let args = Args::parse();
   
   let path = PathBuf::from(args.image_path);
    State::run(Settings { flags: path, ..Default::default() }).unwrap();
}