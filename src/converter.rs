use crate::fileutils::{if_file_exists, run_command};
pub struct ConversionResult {
    pub success: Vec<String>,
    pub fail: Vec<String>,
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
pub fn convert_webp_files(files: &Vec<String>, option: String) -> ConversionResult {
    let mut result: ConversionResult = ConversionResult::new();
    for file in files {
        println!("Converting file: {}", file);
        let command: String;
        let newfile: String;
        if option.len() == 0 {
            newfile = file.replace(".webp", ".png");
            println!("Generating new file: {}", newfile);
            command = format!("dwebp {} {} -o {}", option, file, newfile);
            if if_file_exists(&newfile) {
                result.fail.push(newfile)
            } else {
                run_command(&command);
                if if_file_exists(&newfile) {
                    result.success.push(newfile)
                } else {
                    result.fail.push(newfile)
                }
            }
        } else {
            newfile = file.replace(".webp", format!(".{}", option.replace("-", "")).as_str());
            println!("Generating new file: {}", newfile);
            command = format!("dwebp {} -o {}", file, newfile);
            if if_file_exists(&newfile) {
                result.fail.push(newfile);
            } else {
                run_command(&command);
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