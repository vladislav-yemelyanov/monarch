use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use rootcause::Report;

mod file_manager;
mod find;
mod recent;
mod view;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(long, value_enum, help = "Choose editor (hx or nvim)")]
    editor: Editor,

    #[arg(short, long)]
    projects_dir: Option<PathBuf>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Editor {
    Hx,
    Nvim,
}

fn main() -> Result<(), Report> {
    let cli = Cli::parse();

    // step 1

    // for example:
    // let mut home_dir = PathBuf::from(env::var("HOME")?);
    // home_dir.push("Documents");
    // home_dir.push("pro");

    // let home_dir = dirs::document_dir().expect("Not found Documents, please install MacOS");

    let home_dir = cli.projects_dir.expect("projects_dir is not valid");

    let base_dir = &home_dir.to_str().expect("base_dir failed");

    // step 2
    let mut recent_projects_file = recent::create_recent_projects_file(&home_dir)?;
    let mut paths = find::parse_folders(&home_dir);

    // step 3
    recent::sync_recent_projects_file(&mut recent_projects_file, &paths)?;
    recent::add_recent_projects_to_paths(&home_dir, &mut paths)?;

    // step 4
    view::open_view(&home_dir, &base_dir, &paths, &cli.editor)?;

    Ok(())
}
