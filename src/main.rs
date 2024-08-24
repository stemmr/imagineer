use image::{DynamicImage, GenericImageView, Pixel};
use std::path::Path;

struct AsciiArt {
    image: DynamicImage,
}

impl AsciiArt {
    fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        match image::open(path) {
            Ok(img) => Ok(Self { image: img }),
            Err(e) => Err(format!("Failed to load image: {}", e)),
        }
    }

    fn generate(&self, width: u32) -> String {
        let height = self.image.height() * width / self.image.width();
        let img = self.image.resize_exact(width, height, image::imageops::FilterType::Nearest);

        let mut ascii_art = String::new();
        for y in 0..img.height() {
            for x in 0..img.width() {
                let pixel = img.get_pixel(x, y);
                let brightness = pixel.to_luma().0[0];
                ascii_art.push(Self::brightness_to_char(brightness));
            }
            ascii_art.push('\n');
        }
        ascii_art
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


fn main() {
    let ascii_art = AsciiArt::new("./images/arthur.png").unwrap();
    let ascii_art_string = ascii_art.generate(80);
    println!("{}", ascii_art_string);
}
