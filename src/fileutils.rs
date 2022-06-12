use std::{process::Command};

/// Takes files as arguments presented as a vector of strings and attempts to convert them from .webp to .png
///
/// # Examples
///
/// ```
/// let files : Vec<String> = ["file1.webp", "file2.webp"];
/// convert_webd_files_to_png(files);
///
/// ```
pub fn convert_webd_files_to_png(files : Vec<String>) {
    let mut i : usize = 0;
    while i < files.len() {
        println!("Converting file: {}", files[i]);
        let command : String = format!("dwebp {} -o {}", files[i], files[i].replace(".webp", ".png"));
        Command::new("sh")
            .arg("-c")
            .arg(command.as_str())
            .output()
            .expect(format!("failed to convert file: {}", files[i]).as_str());
            i += 1;
    }
}

/// Gets a list of files from the current working directory and returns them as Vec<String>
pub fn get_files() -> Vec<String> {
    let mut all_files : Vec<String> = String::from_utf8(
        Command::new("sh")
            .arg("-c")
            .arg("ls")
            .output()
            .expect("failed to retrieve current directory listing")
            .stdout,
    )
    .unwrap()
    .split('\n')
    .map(|s| s.to_string())
    .collect();
    let mut i : usize = 0;
    while i < all_files.len() {
        if !all_files[i].ends_with(".webp") {
            all_files.remove(i);
        } else {
            i += 1;
        }
    }
    all_files
}