use imageproc::image::{self, RgbaImage, open};
use rayon::prelude::*;
use serde_yml::from_str;
use std::{collections::HashMap, fs::read_to_string, time::Instant};

fn main() {
    let start = Instant::now();

    // get the image
    let mut image: RgbaImage = load_image("image.png");
    // get the color schemes
    let color_schemes: ColorSchemes = ColorSchemes::new("./src/schemes.yaml");
    let scheme_to_use: Vec<(u8, u8, u8)> = color_schemes.get_scheme("gruvbox");

    recolour_image(&mut image, scheme_to_use);

    // save the image again
    image.save("output.png").expect("Failed to save image");

    let duration = start.elapsed();
    println!("main function took: {:?}", duration);
}

// function to read the image which the path specified leads to
fn load_image(path: &str) -> RgbaImage {
    open(path).expect("Failed to open image").into_rgba8()
}

fn recolour_image(image: &mut RgbaImage, color_scheme: Vec<(u8, u8, u8)>) -> RgbaImage {
    // loop through the pixels in the image
    use rayon::iter::ParallelBridge;
    image
        .pixels_mut()
        .par_bridge()
        .for_each(|pixel: &mut image::Rgba<u8>| {
            let [r_img, g_img, b_img, a] = pixel.0;
            let mut lowest: (u8, u8, u8) = (0, 0, 0);
            let mut lowest_dist: u16 = 442;
            for colour in color_scheme.clone() {
                let current_color_distance: u16 = color_distance((&r_img, &g_img, &b_img), colour);
                if current_color_distance < lowest_dist {
                    lowest = colour;
                    lowest_dist = current_color_distance;
                }
            }

            let (r, g, b) = lowest;
            pixel.0 = [r, g, b, a];
        });
    return image.clone();
}

fn color_distance(color1: (&u8, &u8, &u8), color2: (u8, u8, u8)) -> u16 {
    let (r1, g1, b1) = color1;
    let (r2, g2, b2) = color2;

    (((r2 as i32 - *r1 as i32).pow(2)
        + (g2 as i32 - *g1 as i32).pow(2)
        + (b2 as i32 - *b1 as i32).pow(2)) as f64)
        .sqrt()
        .round() as u16
}

struct ColorSchemes {
    schemes: HashMap<String, Vec<String>>,
}
impl ColorSchemes {
    fn new(file_path: &str) -> ColorSchemes {
        let content: String = read_to_string(file_path).expect("Failed to read schemes.yaml");
        let schemes: HashMap<String, Vec<String>> =
            from_str(&content).expect("Failed to parse YAML");
        ColorSchemes { schemes: schemes }
    }

    fn get_scheme(self: &Self, scheme_name: &str) -> Vec<(u8, u8, u8)> {
        let scheme_to_use = &self.schemes[scheme_name];

        scheme_to_use
            .iter()
            .map(|color| {
                let color = color.trim_start_matches('#');
                let r = u8::from_str_radix(&color[0..2], 16).expect("Invalid red hex");
                let g = u8::from_str_radix(&color[2..4], 16).expect("Invalid green hex");
                let b = u8::from_str_radix(&color[4..6], 16).expect("Invalid blue hex");
                (r, g, b)
            })
            .collect()
    }
}
