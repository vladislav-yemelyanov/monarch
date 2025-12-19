use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

// 💡 ignoring unnecessary folders speeds up searching
static IGNORE_DIRS: &[&str] = &[
    // Package managers and dependencies
    "node_modules",
    "bower_components",
    "vendor",
    "packages",
    "jspm_packages",
    // Version control systems
    ".git",
    ".svn",
    ".hg",
    ".bzr",
    // Build folders and output files
    "build",
    "dist",
    "out",
    "target", // Rust
    "bin",
    "obj",
    ".next",   // Next.js
    ".nuxt",   // Nuxt.js
    ".output", // Nitro
    ".vercel",
    ".netlify",
    // Mobile development
    "android",
    "ios",
    ".expo",
    ".expo-shared",
    // Caches and temporary files
    ".cache",
    ".temp",
    ".tmp",
    "tmp",
    "temp",
    ".parcel-cache",
    ".vite",
    ".turbo",
    // IDEs and editors
    ".vscode",
    ".idea", // JetBrains
    ".vs",   // Visual Studio
    ".fleet",
    ".eclipse",
    ".settings",
    // Language and tool configurations
    ".cargo",
    ".rustup",
    ".npm",
    ".yarn",
    ".pnpm-store",
    ".gradle",
    ".m2", // Maven
    ".nuget",
    "nuget",
    ".composer",   // PHP
    ".bundle",     // Ruby
    "__pycache__", // Python
    ".pytest_cache",
    ".mypy_cache",
    ".ruff_cache",
    "venv",
    ".venv",
    "env",
    ".env",
    // System folders
    ".ssh",
    ".gnupg",
    "Library", // macOS
    "Downloads",
    "Desktop",
    "Pictures",
    "Music",
    "Videos",
    "Movies",
    "Public",
    // Other
    "coverage",
    ".coverage",
    "htmlcov",
    "test-results",
    ".sass-cache",
    ".webpack",
    "Pods",        // iOS CocoaPods
    "DerivedData", // Xcode
];

pub fn find_git_folders(root: &Path, ignore: &HashSet<&str>) -> Vec<DirEntry> {
    WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !ignore.contains(e.file_name().to_str().unwrap_or("")))
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .filter(|e| e.path().join(".git").is_dir())
        // .inspect(|e| println!("{:?}", e.path().to_string_lossy()))
        .collect()
}

pub fn parse_folders(path: &PathBuf) -> Vec<String> {
    let ignore: HashSet<_> = IGNORE_DIRS.iter().copied().collect();

    let paths: Vec<String> = find_git_folders(path.as_path(), &ignore)
        .iter()
        .map(|e| e.path().to_str())
        .filter_map(|e| e.map(|x| x.to_owned()))
        .collect();

    paths
}
