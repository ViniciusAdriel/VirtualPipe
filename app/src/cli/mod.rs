use clap::{Parser, ArgAction};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub restore_only: bool,
}

pub fn parse() -> Cli {
    Cli::parse()
}
