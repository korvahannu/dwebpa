use std::env;

pub enum ImageFormat {
    Png,
    Pam,
    Ppm,
    Bmp,
    Tiff,
    Pgm,
    Yuv
}

impl ImageFormat {
    pub fn to_string(&self) -> String {
        match self {
            ImageFormat::Png => String::from("png"),
            ImageFormat::Pam => String::from("pam"),
            ImageFormat::Ppm => String::from("ppm"),
            ImageFormat::Bmp => String::from("bmp"),
            ImageFormat::Tiff => String::from("tiff"),
            ImageFormat::Pgm => String::from("pgm"),
            ImageFormat::Yuv => String::from("yuv")
        }
    }

    pub fn from_string(format_type: &String) -> Result<ImageFormat, &'static str> {
        let result: Result<ImageFormat, &str>;
        match format_type.as_str() {
            "png" => result = Ok(ImageFormat::Png),
            "pam" => result = Ok(ImageFormat::Pam),
            "ppm" => result = Ok(ImageFormat::Ppm),
            "bmp" => result = Ok(ImageFormat::Bmp),
            "tiff" => result = Ok(ImageFormat::Tiff),
            "pgm" => result = Ok(ImageFormat::Pgm),
            "yuv" => result = Ok(ImageFormat::Yuv),
            _ => return Err("Invalid image format type")
        }
        result
    }
}

pub struct ConverterConfig {
    pub path: String,
    pub recursive: bool,
    pub verbose: bool,
    pub image_format: ImageFormat
}

impl ConverterConfig {
    pub fn new() -> Result<ConverterConfig, &'static str> {
        match env::current_dir() {
            Ok(dir) => {
                match dir.to_str() {
                    Some(dir) => {
                        return Ok(ConverterConfig {
                            path: String::from(dir),
                            recursive: false,
                            verbose: false,
                            image_format: ImageFormat::Png
                        })
                    },
                    None => {
                        return Err("Invalid working directory")
                    }
                }
            },
            Err(_) => {
                return Err("Invalid working directory")
            }
        }
    }
}