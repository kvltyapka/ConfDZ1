use std::fs::File;
use std::io::{self, BufRead, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use zip::read::ZipArchive;

mod commands;
use commands::{
    BaseCommand, CDCommand, ChmodCommand, ClearCommand, ExitCommand, FindCommand, LSCommand,
};

struct ShellEmulator {
    hostname: String,
    fs_path: String,
    log_path: String,
    startup_script: String,
    logger: csv::Writer<File>,
    vfs: Vec<String>,
    current_path: String,
}

impl ShellEmulator {
    fn new(config_path: &str) -> Self {
        let mut emulator = ShellEmulator {
            hostname: String::new(),
            fs_path: String::new(),
            log_path: String::new(),
            startup_script: String::new(),
            logger: csv::Writer::from_writer(File::create("log.csv").unwrap()),
            vfs: Vec::new(),
            current_path: "/".to_string(),
        };
        emulator.load_config(config_path);
        emulator.initialize_vfs(); // Initialize VFS
        emulator
    }

    fn load_config(&mut self, config_path: &str) {
        let file = File::open(config_path).expect("Unable to open config file");
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            let line = line.expect("Unable to read line");
            let parts: Vec<&str> = line.split(',').collect();
            match parts[0] {
                "hostname" => self.hostname = parts[1].to_string(),
                "filesystem_path" => self.fs_path = parts[1].to_string(),
                "log_path" => self.log_path = parts[1].to_string(),
                "startup_script" => self.startup_script = parts[1].to_string(),
                _ => (),
            }
        }
    }

    fn initialize_vfs(&mut self) {
        // Check and load files from ZIP archive
        if let Ok(file) = File::open(&self.fs_path) {
            let mut archive = ZipArchive::new(file).expect("Unable to read ZIP archive");
            for i in 0..archive.len() {
                let file = archive
                    .by_index(i)
                    .expect("Unable to access file in ZIP archive");
                let file_name = format!("/{}", file.name());
                self.vfs.push(file_name);
            }
            println!("VFS initialized with files: {:?}", self.vfs); // Debug
        } else {
            // If ZIP archive not found, add some files to VFS for testing
            println!("ZIP archive not found, initializing default VFS");
            self.vfs.push("/folder1/file1.txt".to_string());
            self.vfs.push("/folder1/file2.txt".to_string());
            self.vfs.push("/folder2/file3.txt".to_string());
        }
    }

    fn execute_command(&mut self, command: &str) -> String {
        let parts: Vec<&str> = command.split_whitespace().collect();
        let cmd = parts[0];
        let args = &parts[1..];
        let result = match cmd {
            "ls" => LSCommand::run(&self.vfs, &self.current_path, args),
            "cd" => {
                let new_path = CDCommand::run(&self.vfs, &self.current_path, args);
                if !new_path.starts_with("Error") {
                    self.current_path = new_path.clone();
                }
                new_path
            }
            "exit" => ExitCommand::run(&self.vfs, &self.current_path, args),
            "find" => FindCommand::run(&self.vfs, &self.current_path, args),
            "clear" => ClearCommand::run(&self.vfs, &self.current_path, args),
            "chmod" => ChmodCommand::run(&self.vfs, &self.current_path, args),
            _ => "Unknown command".to_string(),
        };
        self.log_command(command);
        format!("{}\n", result) // Add newline after each command result
    }

    fn log_command(&mut self, command: &str) {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp = since_the_epoch.as_secs();
        self.logger
            .write_record(&[timestamp.to_string(), command.to_string()])
            .expect("Unable to write log");
    }
}

fn main() {
    let config_file = "config.csv"; // Ensure the file exists
    let mut emulator = ShellEmulator::new(config_file);
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Unable to read line");
        let result = emulator.execute_command(&line);
        print!("{}", result); // Use print instead of println to avoid double newlines
    }
}
