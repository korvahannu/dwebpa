use std::path::PathBuf;
use std::{fs, io};
use std::process::Command;

/// Takes files as arguments presented as a vector of strings deletes them.
///
/// # Examples
///
/// ```
/// let files : Vec<String> = ["file1.webp", "file2.webp"];
/// delete_files(files);
///
/// ```
pub fn delete_files(files: &Vec<PathBuf>) {
    for file in files {
        fs::remove_file(file)
            .unwrap();
    }
}

/// Takes a filename (string) as a parameter.
/// Checks if that file exists in the current working.
/// Returns bool. True if it exists, false if not.
///
/// # Examples
///
/// ```
/// let result : bool = if_file_exists("hello.jpg");
///
/// ```
pub fn if_file_exists(file: &String, path: String) -> bool {
    let all_files = get_non_webp_files(path);

    let mut i: usize = 0;
    while i < all_files.len() {
        if get_filename(&all_files[i]).eq(file) {
            return true
        }
        i += 1;
    }

    false
}

/// Runs a shell command in the current working directory.
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

/// Gets a list of .webp files from the current working directory and returns them as Vec<String>
///
/// # Examples
///
/// ```
/// let web_files: Vec<String> = get_webp_files();
///
/// ```
pub fn get_webp_files(path: String) -> Vec<PathBuf> {
    let mut all_files: Vec<PathBuf> = get_all_files(path);
    println!("All files: {:?}", all_files);
    let mut i: usize = 0;
    while i < all_files.len() {
        if !get_filename(&all_files[i]).ends_with(".webp") {
            all_files.remove(i);
        } else {
            i += 1;
        }
    }
    all_files
}

/// Gets a list of non-.webp files from the current working directory and returns them as Vec<String>
///
/// # Examples
///
/// ```
/// let non_web_files: Vec<String> = get_non_webp_files();
///
/// ```
pub fn get_non_webp_files(path: String) -> Vec<PathBuf> {
    let mut all_files: Vec<PathBuf> = get_all_files(path);
    let mut i: usize = 0;
    while i < all_files.len() {
        if get_filename(&all_files[i]).ends_with(".webp") {
            all_files.remove(i);
        } else {
            i += 1;
        }
    }
    all_files
}

/// Gets a list of files from the current working directory and returns them as Vec<PathBuf>
///
/// # Examples
///
/// ```
/// let all_files: Vec<PathBuf> = get_all_files();
///
/// ```
pub fn get_all_files(path: String) -> Vec<PathBuf> {
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
///
/// # Examples
///
/// ```
/// let all_directories: Vec<PathBuf> = get_all_directories();
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

pub fn get_filename(file: &PathBuf) -> String {
    String::from(file.file_name().unwrap().to_str().unwrap())
}