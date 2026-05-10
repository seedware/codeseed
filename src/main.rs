use clap::Parser;
use codeseed::cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Init(command) => codeseed::init::run(&cli.project, &command).map(|report| {
            println!(
                "Initialized Codeseed in {}",
                codeseed::init::project_display_path(&cli.project)
            );
            println!("created dirs: {}", report.created_dirs.len());
            println!("installed skills: {}", report.installed_skills.len());
            println!("generated files: {}", report.generated_files.len());
            println!("generated links: {}", report.generated_links.len());
        }),
        Command::Add(command) => codeseed::add::run(&cli.project, &command).map(|report| {
            println!("Added skill {}", report.installed_skill);
            println!("generated files: {}", report.generated_files.len());
            println!("generated links: {}", report.generated_links.len());
        }),
        Command::Remove(command) => {
            println!("codeseed rm: project={:?} {command:?}", cli.project);
            Ok(())
        }
        Command::Doctor(command) => {
            println!("codeseed doctor: project={:?} {command:?}", cli.project);
            Ok(())
        }
        Command::Sync(command) => {
            println!("codeseed sync: project={:?} {command:?}", cli.project);
            Ok(())
        }
        Command::Clear(command) => {
            println!("codeseed clear: project={:?} {command:?}", cli.project);
            Ok(())
        }
    };

    if let Err(error) = result {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
