use crate::fileutils::{if_file_exists, run_command, get_filename};
use std::path::PathBuf;
use crate::config::{self, Config};
pub struct ConversionResult {
    pub success: Vec<PathBuf>,
    pub fail: Vec<PathBuf>,
}

impl ConversionResult {
    fn new() -> ConversionResult {
        ConversionResult {
            success: Vec::new(),
            fail: Vec::new(),
        }
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
pub fn convert_webp_files(files: &Vec<PathBuf>, config: Config) -> ConversionResult {
    let mut result: ConversionResult = ConversionResult::new();
    for file in files {
        println!("Converting file: {}", file.display());
        let command: String;
        let newfile: String = get_filename(file).replace(".webp", format!(".{}", config.image_format.to_string()).as_str());
        println!("Generating new file: {}", newfile);
        command = format!("dwebp {} -o {}", file.display(), newfile);
        if if_file_exists(&newfile, String::from(".")) {
            result.fail.push(file.clone());
        } else {
            run_command(&command);
            if if_file_exists(&newfile, String::from(".")) {
                result.success.push(file.clone())
            } else {
                result.fail.push(file.clone())
            }
        }
    }
    result
}