pub enum ImageFormat {
    png,
    pam,
    ppm,
    bmp,
    tiff,
    pgm,
    yuv
}

impl ImageFormat {
    pub fn to_string(&self) -> String {
        match self {
            ImageFormat::png => String::from("png"),
            ImageFormat::pam => String::from("pam"),
            ImageFormat::ppm => String::from("ppm"),
            ImageFormat::bmp => String::from("bmp"),
            ImageFormat::tiff => String::from("tiff"),
            ImageFormat::pgm => String::from("pgm"),
            ImageFormat::yuv => String::from("yuv")
        }
    }
}

pub struct Config {
    pub path: String,
    pub recursive: bool,
    pub verbose: bool,
    pub image_format: ImageFormat
}

impl Config {
    pub fn new() -> Config {
        Config {
            path: String::from("."),
            recursive: false,
            verbose: false,
            image_format: ImageFormat::png
        }
    }
}