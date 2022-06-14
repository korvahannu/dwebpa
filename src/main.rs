use dwebpa::fileutils::{get_files, convert_webd_files_to_png};
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
    let files: Vec<String> = get_files();
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
        convert_webd_files_to_png(files, option);
    }
}