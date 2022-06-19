use crate::{fileutils::{run_command, get_webp_files, delete_files}, converterconfig::ImageFormat};
use std::path::PathBuf;
use crate::converterconfig::ConverterConfig;
/// A simple struct that holds information about two Vec<PathBuf> values
/// Converter -struct uses this to save successful conversions to ConversionResult.success
/// and failed conversion to ConversionResult.fail
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

/// A Converter that is used to convert multiple .webp files into another format
/// Users builder pattern
///
/// # Examples
///
/// ```
/// let converter = Converter::new().unwrap();
/// converter.load_files();
/// converter.config = Config {
///     path: String::from("."),
///     recursive: false,
///     verbose: false,
///     image_format: ImageFormat::Png,
/// }
/// converter.convert_files();
/// converter.delete_converted_files();
///
/// ```
pub struct Converter {
    pub files: Vec<PathBuf>,
    pub config: ConverterConfig,
    pub conversion_result: ConversionResult
}

impl Converter {
    pub fn new() -> Result<Converter, &'static str> {
        match ConverterConfig::new() {
            Ok(config) => {
                return Ok(Converter {
                    config,
                    files: Vec::new(),
                    conversion_result: ConversionResult::new()
                })
            },
            Err(msg) => {
                return Err(msg);
            }
        }
    }
    pub fn print_files(&self) {
        for file in &self.files {
            println!("{}", file.display());
        }
    }
    pub fn print_converted_files(&self) {
        for file in &self.conversion_result.success {
            println!("{}", file.display());
        }
    }
    pub fn print_failed_files(&self) {
        for file in &self.conversion_result.fail {
            println!("{}", file.display());
        }
    }
    pub fn load_files(&mut self) {
        self.log(String::from("Scanning for .webp files..."));
        self.files = get_webp_files(&self.config.path, self.config.recursive);
    }
    pub fn convert_files(&mut self) {
        self.conversion_result = ConversionResult::new();
        for file in &self.files {
            self.log(format!("Converting file: {}", file.display()));
            let command: String;
            let newfile: String = file.display().to_string().clone().replace(".webp", format!(".{}", self.config.image_format.to_string()).as_str());
            self.log(format!("Generating new file: {}", newfile));

            if matches!(self.config.image_format, ImageFormat::Png) {
                command = format!("dwebp {} -o {}", file.display(), newfile);
            } else {
                command = format!("dwebp -{} {} -o {}", self.config.image_format.to_string(), file.display(), newfile);
            }

            if std::path::Path::new(&newfile).exists() {
                self.conversion_result.fail.push(file.clone());
            } else {
                run_command(&command);
                if std::path::Path::new(&newfile).exists() {
                    self.conversion_result.success.push(file.clone());
                } else {
                    self.conversion_result.fail.push(file.clone());
                }
            }
        }
    }
    pub fn delete_converted_files(&self) {
        delete_files(&self.conversion_result.success);
    }
    fn log(&self, message: String) {
        if self.config.verbose {
            println!("{}", message);
        }
    }
}