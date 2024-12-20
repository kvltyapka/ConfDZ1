use std::collections::HashSet;

pub trait BaseCommand {
    fn run(vfs: &Vec<String>, current_path: &str, args: &[&str]) -> String;
}

pub struct LSCommand;

impl BaseCommand for LSCommand {
    fn run(vfs: &Vec<String>, current_path: &str, _args: &[&str]) -> String {
        let mut files = HashSet::new();
        let last_folder = current_path
            .split('/')
            .filter(|s| !s.is_empty())
            .last()
            .unwrap_or("");
        for file_path in vfs {
            if file_path.starts_with(current_path) {
                let paths: Vec<&str> = file_path.split('/').collect();
                if let Some(index) = paths.iter().position(|&x| x == last_folder) {
                    if let Some(file) = paths.get(index + 1) {
                        files.insert(*file);
                    }
                }
            }
        }
        files
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("\n")
    }
}

pub struct CDCommand;

impl BaseCommand for CDCommand {
    fn run(vfs: &Vec<String>, current_path: &str, args: &[&str]) -> String {
        if args.is_empty() {
            return "Error: not enough arguments".to_string();
        }
        match args[0] {
            ".." => {
                let paths: Vec<&str> = current_path.split('/').filter(|s| !s.is_empty()).collect();
                if paths.len() <= 1 {
                    "/".to_string()
                } else {
                    let new_path = paths[..paths.len() - 1].join("/");
                    format!("/{}", new_path)
                }
            }
            "/" => "/".to_string(),
            new_path => {
                let new_path = format!("{}/{}", current_path.trim_end_matches('/'), new_path);
                for file_path in vfs {
                    if file_path.starts_with(&new_path) {
                        return format!("{}/", new_path);
                    }
                }
                "Error: path not found".to_string()
            }
        }
    }
}

pub struct ExitCommand;

impl BaseCommand for ExitCommand {
    fn run(_vfs: &Vec<String>, _current_path: &str, _args: &[&str]) -> String {
        "exit".to_string()
    }
}

pub struct FindCommand;

impl BaseCommand for FindCommand {
    fn run(vfs: &Vec<String>, _current_path: &str, args: &[&str]) -> String {
        if args.is_empty() {
            return "Error: not enough arguments".to_string();
        }
        for file_path in vfs {
            if file_path.contains(args[0]) {
                return file_path.clone();
            }
        }
        "Error: path not found".to_string()
    }
}

pub struct ClearCommand;

impl BaseCommand for ClearCommand {
    fn run(_vfs: &Vec<String>, _current_path: &str, _args: &[&str]) -> String {
        "clear".to_string()
    }
}

pub struct ChmodCommand;

impl BaseCommand for ChmodCommand {
    fn run(vfs: &Vec<String>, _current_path: &str, args: &[&str]) -> String {
        if args.len() < 2 {
            return "Error: not enough arguments".to_string();
        }
        let file_path = args[1];
        if vfs.contains(&file_path.to_string()) {
            format!("Changed permissions for {}", file_path)
        } else {
            "Error: file not found".to_string()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn setup_vfs() -> Vec<String> {
        vec![
            "/folder1/file1.txt".to_string(),
            "/folder1/file2.txt".to_string(),
            "/folder2/file3.txt".to_string(),
        ]
    }

    #[test]
    fn test_ls_lists_files_in_current_directory() {
        let vfs = setup_vfs();
        let result = LSCommand::run(&vfs, "/folder1/", &[]);
        assert!(result.contains("file1.txt"));
        assert!(result.contains("file2.txt"));
        assert!(!result.contains("file3.txt"));
    }

    #[test]
    fn test_cd_to_subdirectory() {
        let vfs = setup_vfs();
        let result = CDCommand::run(&vfs, "/", &["folder1"]);
        assert_eq!(result, "/folder1/");
    }

    #[test]
    fn test_cd_to_parent_directory() {
        let vfs = setup_vfs();
        let result = CDCommand::run(&vfs, "/folder1/", &[".."]);
        assert_eq!(result, "/");
    }

    #[test]
    fn test_cd_non_existent_directory() {
        let vfs = setup_vfs();
        let result = CDCommand::run(&vfs, "/", &["non_existent_folder"]);
        assert_eq!(result, "Error: path not found");
    }

    #[test]
    fn test_exit_command() {
        let vfs = setup_vfs();
        let result = ExitCommand::run(&vfs, "/", &[]);
        assert_eq!(result, "exit");
    }

    #[test]
    fn test_find_file_in_vfs() {
        let vfs = setup_vfs();
        let result = FindCommand::run(&vfs, "/", &["file1.txt"]);
        assert_eq!(result, "/folder1/file1.txt");
    }

    #[test]
    fn test_find_file_not_found() {
        let vfs = setup_vfs();
        let result = FindCommand::run(&vfs, "/", &["nonexistent_file.txt"]);
        assert_eq!(result, "Error: path not found");
    }

    #[test]
    fn test_find_without_arguments() {
        let vfs = setup_vfs();
        let result = FindCommand::run(&vfs, "/", &[]);
        assert_eq!(result, "Error: not enough arguments");
    }

    #[test]
    fn test_clear_command() {
        let vfs = setup_vfs();
        let result = ClearCommand::run(&vfs, "/", &[]);
        assert_eq!(result, "clear");
    }

    #[test]
    fn test_chmod_command() {
        let vfs = setup_vfs();
        let result = ChmodCommand::run(&vfs, "/", &["755", "/folder1/file1.txt"]);
        assert_eq!(result, "Changed permissions for /folder1/file1.txt");
    }

    #[test]
    fn test_chmod_file_not_found() {
        let vfs = setup_vfs();
        let result = ChmodCommand::run(&vfs, "/", &["755", "/nonexistent_file.txt"]);
        assert_eq!(result, "Error: file not found");
    }

    #[test]
    fn test_chmod_not_enough_arguments() {
        let vfs = setup_vfs();
        let result = ChmodCommand::run(&vfs, "/", &["755"]);
        assert_eq!(result, "Error: not enough arguments");
    }
}
