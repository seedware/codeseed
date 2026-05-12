use clap::Parser;
use codeseed::cli::{Cli, Command, OutputFormat};

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
        Command::List(command) => codeseed::list::run(&cli.project, &command).map(|report| {
            let output = match command.format {
                OutputFormat::Text => codeseed::list::format_text(&report),
                OutputFormat::Json => codeseed::list::format_json(&report),
            };
            print!("{output}");
        }),
        Command::Update(command) => codeseed::update::run(&cli.project, &command).map(|report| {
            if report.executed {
                println!("Updated Codeseed");
            } else {
                println!("Update plan: {}", report.plan);
            }
        }),
        Command::Remove(command) => codeseed::remove::run(&cli.project, &command).map(|report| {
            println!("Removed skill {}", report.removed_skill);
            println!("removed paths: {}", report.removed_paths.len());
            println!("pruned dirs: {}", report.pruned_dirs.len());
            println!(
                "updated state: {}",
                if report.updated_state { "yes" } else { "no" }
            );
        }),
        Command::Doctor(command) => {
            println!("codeseed doctor: project={:?} {command:?}", cli.project);
            Ok(())
        }
        Command::Sync(command) => {
            println!("codeseed sync: project={:?} {command:?}", cli.project);
            Ok(())
        }
        Command::Clear(command) => codeseed::clear::run(&cli.project, &command).map(|report| {
            if report.dry_run {
                println!("Clear plan: {} paths", report.planned_paths.len());
            } else {
                println!("Cleared Codeseed-managed state");
                println!("removed paths: {}", report.removed_paths.len());
                println!("pruned dirs: {}", report.pruned_dirs.len());
            }
        }),
    };

    if let Err(error) = result {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
