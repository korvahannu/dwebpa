use dwebpa::fileutils::{get_webp_files, convert_webp_files, delete_files, ConversionResult};
use std::io::stdin;
use std::env;

fn main() {
    if cfg!(target_os = "windows") {
        panic!("Dweba supports linux only")
    };
    let mut option : String = String::new();
    for argument in env::args() {
        if argument.starts_with("-") {
            option = argument;
            println!("[DWEBA] Set {} as webp option", option);
            break;
        }
    }
    let files: Vec<String> = get_webp_files();
    println!("[DWEBA] The following files will be converted from .webp to .png:");
    let mut i : usize = 0;
    while i < files.len() {
        println!("{}", files[i]);
        i += 1;
    }
    println!("Do you wish to continue? [Y/n]");
    let mut c : String = String::new();
    stdin().read_line(&mut c).unwrap();
    if c.as_str().chars().nth(0).unwrap() != 'n' {
        let result: ConversionResult = convert_webp_files(&files, option);
        if result.fail.len() > 0 {
            println!("The following files failed to convert:");
            for file in result.fail {
                println!("{}", file);
            }
            println!("However, the following files DID convert successfully:");
            for file in &result.success {
                println!("{}", file);
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