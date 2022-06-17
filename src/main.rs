use dwebpa::fileutils::{get_webp_files, delete_files};
use dwebpa::converter::{convert_webp_files, ConversionResult};
use dwebpa::config::Config;
use std::io::stdin;
use std::path::PathBuf;

fn main() {
    if cfg!(target_os = "windows") {
        panic!("Dweba supports linux only")
    };
    let config : Config = Config::new();
    let files: Vec<PathBuf> = get_webp_files(String::from("."));
    println!("[DWEBA] The following files will be converted from .webp to .png:");
    let mut i : usize = 0;
    while i < files.len() {
        println!("{}", files[i].display());
        i += 1;
    }
    println!("Do you wish to continue? [Y/n]");
    let mut c : String = String::new();
    stdin().read_line(&mut c).unwrap();
    if c.as_str().chars().nth(0).unwrap() != 'n' {
        let result: ConversionResult = convert_webp_files(&files, config);
        if result.fail.len() > 0 {
            println!("The following files failed to convert:");
            for file in result.fail {
                println!("{}", file.display());
            }
            println!("However, the following files DID convert successfully:");
            for file in &result.success {
                println!("{}", file.display());
            }
            println!("Do you wish to delete the successfully converted .webp files? [y/N]");
            c = String::new();
            stdin().read_line(&mut c).unwrap();
            if c.as_str().chars().nth(0).unwrap() == 'Y' {
                delete_files(&result.success);
            }
        } else {
            println!("Conversion completed! Do you wish to delete the .webp files? [y/N]");
            c = String::new();
            stdin().read_line(&mut c).unwrap();
            if c.as_str().chars().nth(0).unwrap() == 'Y' {
                delete_files(&files);
            }
        }
    }
}