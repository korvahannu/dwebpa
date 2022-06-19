use dwebpa::converter::Converter;
use dwebpa::dwebpaconfig::DwebpaConfig;
use dwebpa::parsearguments::parse_arguments;
use dwebpa::utils::{ask_question};
use std::env;

fn main() {
    if cfg!(target_os = "windows") {
        panic!("Dweba supports linux only")
    };
    
    if !does_system_have_dwebp() {
        println!("dwebp not found in system
        
        You can install this via your preferred package manager:
        sudo apt install webp
        sudo pacman -S libwebp
        sudo zypper install libwebp-tools");
        return
    }
    
    let mut converter: Converter;

    match Converter::new() {
        Ok(conv) => {
            converter = conv;
        },
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    }

    let dwebpa_config;
    
    match parse_arguments() {
        Ok((d, c)) => {
            dwebpa_config = d;
            converter.config = c;
        },
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    }
    
    converter.load_files();
    if converter.files.len() == 0 {
        log(&dwebpa_config,"Nothing to convert at specified path! Try appending your command with --help for more information.");
        return;
    }
    log(&dwebpa_config, format!("[DWEBA] The following files will be converted from .webp to .{}:", converter.config.image_format.to_string()).as_str());
    if !dwebpa_config.quiet {
        converter.print_files();
    }
    if dwebpa_config.auto_accept_conversion || ask_question(String::from("Do you wish to continue? [Y/n]")) != 'n' {
        log(&dwebpa_config,"Converting files...");
        converter.convert_files();
        if converter.conversion_result.fail.len() > 0 {
            println!("The following files failed to convert:");
            converter.print_failed_files();
            if converter.conversion_result.success.len() > 0 {
                println!("However, the following files DID convert successfully:");
                converter.print_converted_files();
                if !dwebpa_config.auto_decline_cleanup {
                    if dwebpa_config.auto_accept_cleanup || ask_question(String::from("Do you wish to delete the successfully converted .webp files? [y/N]")) == 'Y' {
                        converter.delete_converted_files();
                    }
                }
            }
        } else if !dwebpa_config.auto_decline_cleanup {
            if dwebpa_config.auto_accept_cleanup || ask_question(String::from("Conversion completed! Do you wish to delete the .webp files? [y/N]")) == 'Y' {
                converter.delete_converted_files();
            }
        }
    }
}

// Uses the system $PATH variable and checks if the dwebp binary exists in the system
// returns false if the binary does not exist
fn does_system_have_dwebp() -> bool {
    let env_var_path = env::var("PATH").unwrap();
    let paths: Vec<&str> = env_var_path.split(":").collect();

    for path in paths {
        if std::path::Path::new(format!("{}/dwebp", path).as_str()).exists() {
            return true;
        }
    }

    false
}

fn log(config: &DwebpaConfig, message: &str) {
    if !config.quiet {
        println!("{}", message);
    }
}