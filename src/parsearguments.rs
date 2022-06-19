use crate::converterconfig::{ConverterConfig, ImageFormat};
use crate::dwebpaconfig::DwebpaConfig;
use std::env;
use std::path::PathBuf;

pub fn parse_arguments() -> Result<(DwebpaConfig, ConverterConfig), &'static str> {
    let mut dwebpa_config: DwebpaConfig = DwebpaConfig::new();
    let mut converter_config: ConverterConfig;

    match ConverterConfig::new() {
        Ok(config) => {
            converter_config = config;
        },
        Err(msg) => {
            return Err(msg);
        }
    }

    let mut arguments = env::args().into_iter();
    arguments.next();
    let mut t_switch = false;

    while let Some(argument) = arguments.next() {

        if t_switch {
            match ImageFormat::from_string(&argument) {
                Ok(x) => converter_config.image_format = x,
                Err(msg) => {
                    return Err(msg);
                }
            }
            t_switch = false;
            continue;
        }

        if argument.starts_with("--") {
            match argument.as_str() {
                "--quiet" => dwebpa_config.quiet = true,
                "--auto-accept-conversion" => dwebpa_config.auto_accept_conversion = true,
                "--auto-accept-cleanup" => dwebpa_config.auto_accept_cleanup = true,
                "--verbose" => converter_config.verbose = true,
                "--recursive" => converter_config.recursive = true,
                "--type" => t_switch = true,
                "--help" => return Err("dwebpa is a commandline tool to convert your .webp file into another format
            
                dwebpa [options] [path]
    
                -q / --quiet
                -y / --auto-accept-conversion
                -c / --auto-decline-cleanup (overwrites -C)
                -C / --auto-accept-cleanup
                -v / --verbose
                -R / --recursive
                -t / --type 
            
                Examples:
                dwebpa -qaARt tiff /home/jack
                dwebpa --type png --auto-accept-conversion"),
                _ => ()
            }
        } else if argument.starts_with('-') {
            let arg = argument.chars();
            for c in arg {
                match &c {
                    'q' => dwebpa_config.quiet = true,
                    'y' => dwebpa_config.auto_accept_conversion = true,
                    'c' => dwebpa_config.auto_decline_cleanup = true,
                    'C' => dwebpa_config.auto_accept_cleanup = true,
                    'v' => converter_config.verbose = true,
                    'R' => converter_config.recursive = true,
                    't' => t_switch = true,
                    _ => (),
                }
            }
        } else if argument.starts_with("/") || argument.starts_with(".") {

            if arguments.next().is_some() {
                return Err("Invalid command format.
                Please use it as: dwebpa [options] [path]
                You can append the command with --help for more information");
            }

            let path = PathBuf::from(argument).canonicalize().unwrap();
            if path.is_dir() {
                converter_config.path = path.display().to_string();
            }
        }
    }

    Ok((dwebpa_config, converter_config))
}