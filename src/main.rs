use clap::Parser;
use codeseed::cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Init(command) => {
            println!("codeseed init: project={:?} {command:?}", cli.project);
        }
        Command::Add(command) => {
            println!("codeseed add: project={:?} {command:?}", cli.project);
        }
        Command::Remove(command) => {
            println!("codeseed rm: project={:?} {command:?}", cli.project);
        }
        Command::Doctor(command) => {
            println!("codeseed doctor: project={:?} {command:?}", cli.project);
        }
        Command::Sync(command) => {
            println!("codeseed sync: project={:?} {command:?}", cli.project);
        }
    }
}
