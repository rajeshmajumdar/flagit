use std::fs;
use std::io::{BufRead, BufReader};

pub fn is_git_repo(dir: &std::path::Path) -> bool {
    let path = dir.join(".git");
    fs::metadata(path).is_ok()
}

fn is_ignored(path: &std::path::PathBuf) -> bool {
    if let Ok(contents) = fs::read_to_string(".gitignore") {
        for line in contents.lines() {
            let line = line.trim();
            if line.starts_with("#") || line.is_empty() {
                continue;
            }
            let pattern = line.replace("**", ".*").replace("*", "[^/]*");
            if path.to_string_lossy().contains(&pattern) {
                return true;
            } else if path.to_string_lossy().contains(".git") {
                return true;
            }
        }
    }
    false
}

pub fn get_all_files(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut result = Vec::new();
    let mut files: Vec<std::path::PathBuf> = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            result.push(path);
        } else {
            result.extend(get_all_files(&path));
        }
    }
    for file in result {
        if !is_ignored(&file) {
            files.push(file);
        }
    }
    files
}

pub fn get_issues(path: &std::path::Path) -> Vec<String> {
    let mut issues = Vec::new();
    let file = fs::File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains("TODO") || line.contains("FIXME") {
            issues.push(line);
        }
    }

    issues
}