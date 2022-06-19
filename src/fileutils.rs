use std::path::PathBuf;
use std::process::Command;
use std::{fs, io};
use walkdir::WalkDir;

/// Takes files as arguments presented as a vector of strings deletes them.
///
/// # Examples
///
/// ```
/// let files : Vec<PathBuf> = vec![PathBuf::new("./example1.png"), PathBuf::new("./example2.png")];
/// delete_files(&files);
///
/// ```
pub fn delete_files(files: &Vec<PathBuf>) {
    for file in files {
        match fs::remove_file(file) {
            Ok(_) => {
                continue;
            },
            Err(error) => {
                println!("Failed to remove file {}", file.display());
                println!("{}", error);
            }
        }
    }
}

/// Runs a shell command in the specified path
///
/// # Examples
///
/// ```
/// run_command(&String::from("rm -rf ./*"));
///
/// ```
pub fn run_command(command: &String) {
    Command::new("sh")
        .arg("-c")
        .arg(command.as_str())
        .output()
        .expect(format!("failed to execute command: {}", command).as_str());
}

/// Gets a list of .webp files from the current working directory and returns them as Vec<PathBuf>
/// Second parameter (bool) is a switch. False means get from the given path only. True means
/// get the files recursively.
///
/// # Examples
///
/// ```
/// let web_files: Vec<PathBuf> = get_webp_files("/home/example", false);
///
/// ```
pub fn get_webp_files(path: &String, recursive: bool) -> Vec<PathBuf> {
    let mut all_files: Vec<PathBuf>;
    if recursive {
        all_files = get_all_files_recursive(path)
    } else {
        all_files = get_all_files(path);
    }
    let mut i: usize = 0;
    while i < all_files.len() {
        if !all_files[i].display().to_string().ends_with(".webp") {
            all_files.remove(i);
        } else {
            i += 1;
        }
    }
    all_files
}

pub fn get_all_files_recursive(path: &String) -> Vec<PathBuf> {
    let scan: WalkDir = WalkDir::new(path);
    let mut result: Vec<PathBuf> = Vec::new();
    for file in scan {
        result.push(PathBuf::from(file.unwrap().path()));
    }

    result
}

/// Gets a list of non-.webp files from the specified directory and returns them as Vec<PathBuf>
/// Takes in parameter path: &String
///
/// # Examples
///
/// ```
/// let non_web_files: Vec<String> = get_non_webp_files();
///
/// ```
pub fn get_non_webp_files(path: &String) -> Vec<PathBuf> {
    let mut all_files: Vec<PathBuf> = get_all_files(path);
    let mut i: usize = 0;
    while i < all_files.len() {
        if all_files[i].display().to_string().ends_with(".webp") {
            all_files.remove(i);
        } else {
            i += 1;
        }
    }
    all_files
}

/// Gets a list of files from the specified directory and returns them as Vec<PathBuf>
/// Takes in parameter path: &String
///
/// # Examples
///
/// ```
/// let all_files: Vec<PathBuf> = get_all_files();
///
/// ```
pub fn get_all_files(path: &String) -> Vec<PathBuf> {
    let mut entries = fs::read_dir(path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .filter(|entry| entry.as_ref().unwrap().is_file())
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
    entries.sort();
    entries
}

/// Gets a list of directories from the specified path and returns them as Vec<PathBuf>
/// Takes in parameter path: &String
///
/// # Examples
///
/// ```
/// let all_directories: Vec<PathBuf> = get_all_directories("/home/example");
///
/// ```
pub fn get_all_directories(path: String) -> Vec<PathBuf> {
    let mut entries = fs::read_dir(path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .filter(|entry| entry.as_ref().unwrap().is_dir())
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
    entries.sort();
    entries
}