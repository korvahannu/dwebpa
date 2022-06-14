use std::{process::Command};

pub struct ConversionResult {
    pub success: Vec<String>,
    pub fail: Vec<String>
}

/// Takes files as arguments presented as a vector of strings deletes them
///
/// # Examples
///
/// ```
/// let files : Vec<String> = ["file1.webp", "file2.webp"];
/// delete_files(files);
///
/// ```
pub fn delete_files(files: &Vec<String>) {
    for file in files {
        Command::new("sh")
            .arg("-c")
            .arg(format!("rm ./{}", file))
            .output()
            .expect(format!("failed to delete file: {}", file).as_str());
    }
}

/// Takes files as arguments presented as a vector of strings and attempts to convert them from .webp to .png
///
/// # Examples
///
/// ```
/// let files : Vec<String> = ["file1.webp", "file2.webp"];
/// convert_webd_files_to_png(files);
///
/// ```
pub fn convert_webp_files(files : &Vec<String>, option: String) -> ConversionResult {
    let mut result: ConversionResult = ConversionResult {
        success: Vec::new(),
        fail: Vec::new()
    };
    for file in files {
        println!("Converting file: {}", file);
        let command : String;
        if option.len() == 0 {
            let newfile: String = file.replace(".webp", ".png");
            println!("Generating new file: {}", newfile);
            command = format!("dwebp {} {} -o {}", option, file, newfile);
            if if_file_exists(&newfile) {
                result.fail.push(newfile)
            } else {
                convert_webp_file(file, command);

                if if_file_exists(&newfile) {
                    result.success.push(newfile)
                } else {
                    result.fail.push(newfile)
                }
            }
        } else {
            let newfile: String = file.replace(".webp", format!(".{}", option.replace("-", "")).as_str());
            println!("Generating new file: {}", newfile);
            command = format!("dwebp {} -o {}", file, newfile);
            if if_file_exists(&newfile) {
                result.fail.push(newfile);
            } else {
                convert_webp_file(file, command);
                if if_file_exists(&newfile) {
                    result.success.push(newfile)
                } else {
                    result.fail.push(newfile)
                }
            }
        }
    }
    result
}

fn if_file_exists(file: &String) -> bool {
    let all_files = get_non_webp_files();
    if all_files.contains(file) {
        return true
    }
    false
}

fn convert_webp_file(file: &String, command: String) {
    Command::new("sh")
    .arg("-c")
    .arg(command.as_str())
    .output()
    .expect(format!("failed to convert file: {}", file).as_str());
}

/// Gets a list of .webp files from the current working directory and returns them as Vec<String>
pub fn get_webp_files() -> Vec<String> {
    let mut all_files : Vec<String> = get_all_files();
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

/// Gets a list of non-.webp files from the current working directory and returns them as Vec<String>
pub fn get_non_webp_files() -> Vec<String> {
    let mut all_files : Vec<String> = get_all_files();
    let mut i : usize = 0;
    while i < all_files.len() {
        if all_files[i].ends_with(".webp") {
            all_files.remove(i);
        } else {
            i += 1;
        }
    }
    all_files
}

/// Gets a list of files from the current working directory and returns them as Vec<String>
pub fn get_all_files() -> Vec<String> {
    String::from_utf8(
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
    .collect()
}