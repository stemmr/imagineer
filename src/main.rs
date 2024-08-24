use image::{DynamicImage, GenericImageView, Pixel};
use std::path::Path;
use colored::Colorize;
use clap::Parser;

struct Imagineer {
    image: DynamicImage,
}

impl Imagineer {
    fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        match image::open(path) {
            Ok(img) => Ok(Self { image: img }),
            Err(e) => Err(format!("Failed to load image: {}", e)),
        }
    }

    fn generate(&self, width: u32) -> String {
        let height = self.image.height() * width / self.image.width();
        let img = self.image.resize_exact(width, height / 2, image::imageops::FilterType::Gaussian);

        let mut ascii_str = String::new();
        for y in 0..img.height() {
            for x in 0..img.width() {
                let pixel = img.get_pixel(x, y);
                let brightness = pixel.to_luma().0[0];
                let color = pixel.to_rgb();
                let colored_char = Self::brightness_to_char(brightness)
                    .to_string().truecolor(color.0[0], color.0[1], color.0[2]);
                ascii_str.push_str(&format!("{}", colored_char));
            }
            ascii_str.push('\n');
        }
        ascii_str
    }

    fn brightness_to_char(brightness: u8) -> char {
        match brightness {
            0..=25 => '@',
            26..=50 => '#',
            51..=75 => '8',
            76..=100 => '&',
            101..=125 => 'o',
            126..=150 => ':',
            151..=175 => '*',
            176..=200 => '.',
            201..=225 => ' ',
            _ => ' ',
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "./images/arthur.png")]
    image_path: String,
    #[arg(short, long, default_value = "80")]
    width: u32,
}


fn main() {

    let args = Args::parse();

    let image_path = args.image_path;
    let width = args.width;

    let imagineer = Imagineer::new(image_path).unwrap();
    let imagineer_art_string = imagineer.generate(width);
    println!("Art String: \n{}", imagineer_art_string);
}
