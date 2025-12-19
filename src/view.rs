use std::{
    io::{self, Cursor},
    path::PathBuf,
};

use skim::{
    options::SkimOptionsBuilderError,
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim, SkimOptions,
};

use crate::{file_manager, recent};

#[derive(Debug, thiserror::Error)]
pub enum OpenViewError {
    #[error("recent projects file not received")]
    FileNotReceived(#[from] io::Error),

    #[error("skim run failed")]
    SkimRunFailed,

    #[error("skim create options failed")]
    SkimOptionsFailed(#[from] SkimOptionsBuilderError),
}

fn create_options(base_path: &str) -> Result<SkimOptions, SkimOptionsBuilderError> {
    SkimOptionsBuilder::default()
        .preview(Some(
            format!("eza --icons -L 1 '{}/{{}}'", base_path).to_string(),
        ))
        .preview_window("right:30:wrap".to_string())
        .build()
}

pub fn open_view(
    home_dir: &PathBuf,
    base_path: &str,
    result: &Vec<String>,
) -> Result<(), OpenViewError> {
    let mut file = recent::create_recent_projects_file(home_dir)?;

    let options = create_options(&base_path)?;

    let normalized_paths: Vec<String> = result
        .iter()
        .map(|path| path.strip_prefix(base_path).unwrap_or(path).to_string())
        .collect();

    let input = normalized_paths.join("\n");

    let item_reader = SkimItemReader::default();

    let items = item_reader.of_bufread(Cursor::new(input));

    let selected = Skim::run_with(&options, Some(items));

    let restored = match selected {
        Some(out) if !out.is_abort => {
            let selected_item = out
                .selected_items
                .first()
                .map(|item| item.output().to_string())
                .ok_or(OpenViewError::SkimRunFailed)?;

            if selected_item.starts_with(base_path) {
                selected_item
            } else {
                format!("{}{}", base_path, selected_item)
            }
        }
        _ => {
            return Ok(()); // exit
        }
    };

    {
        // LRU

        let mut lines = file_manager::read(&file)?;

        lines.retain(|line| *line != *restored); // remove selected path

        lines.push(restored.clone()); // add selected path to end

        file_manager::write(&mut file, &lines)?;

        println!("{}", restored);
    }

    Ok(())
}
