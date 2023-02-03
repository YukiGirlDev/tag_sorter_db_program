use std::path::PathBuf;

use clap::Parser;


#[derive(Parser)]
#[command(author="YukiGirlDev", version, about, long_about = None)]
pub struct Cli {
    pub path: PathBuf
}