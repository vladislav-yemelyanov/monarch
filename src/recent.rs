use crate::file_manager;
use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, thiserror::Error)]
pub enum RecentError {
    #[error("recent projects file not received")]
    FileNotReceived(#[from] io::Error),
}

pub fn create_recent_projects_file(home_dir: &PathBuf) -> Result<File, io::Error> {
    let recent_file = ".recent_projects";

    let file_path = home_dir.join(recent_file);

    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(file_path)?;

    Ok(file)
}

pub fn add_recent_projects_to_paths(
    home_dir: &PathBuf,
    paths: &mut Vec<String>,
) -> Result<(), RecentError> {
    let recent_projects_file = create_recent_projects_file(&home_dir)?;

    let lines = file_manager::read(&recent_projects_file)?;
    let recent_lines: HashSet<&String> = lines.iter().collect();

    paths.retain(|p| !recent_lines.contains(p));
    paths.extend(lines);
    paths.reverse();

    Ok(())
}

pub fn sync_recent_projects_file(
    recent_projects_file: &mut File,
    paths: &[String],
) -> io::Result<()> {
    let cloned = recent_projects_file.try_clone()?;
    let reader = BufReader::new(cloned);
    let file_lines: Vec<String> = reader.lines().collect::<io::Result<Vec<_>>>()?;

    let paths_set: HashSet<&String> = paths.iter().collect();

    let lines: Vec<String> = file_lines
        .into_iter()
        .filter(|line| paths_set.contains(line)) // O(1)
        .collect();

    file_manager::write(recent_projects_file, &lines)?;
    Ok(())
}
