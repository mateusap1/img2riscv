extern crate image;

use clap::Parser;
use image::{DynamicImage, GenericImageView, Rgba};
use std::fs;

fn pixel_transform(pixel: Rgba<u8>) -> u8 {
    if pixel[3] == 0 {
        return 0xC7;
    }

    let red_big: u32 = (7 * (pixel[0] as u32)) / 255;
    let red: u8 = red_big as u8;

    let green_big: u32 = (7 * (pixel[1] as u32)) / 255;
    let green: u8 = green_big as u8;

    let blue_big: u32 = (3 * (pixel[2] as u32)) / 255;
    let blue: u8 = blue_big as u8;

    let color_transformed: u8 = red + (green << 3) + (blue << 6);
    if color_transformed == 0xC7 {
        0xC6 // Prevent transparent colors by accident
    } else {
        color_transformed
    }
}

fn image_to_riscv(image: DynamicImage, dimensions: (u32, u32)) -> Vec<u8> {
    let mut image_array: Vec<u8> = Vec::new();

    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            // Get the RGBA values of the pixel at (x, y)
            let pixel: Rgba<u8> = image.get_pixel(x, y);
            image_array.push(pixel_transform(pixel));
        }
    }

    image_array
}

fn image_array_to_string(name: &str, image_array: Vec<u8>, dimensions: (u32, u32)) -> String {
    let string_image_array: Vec<String> = image_array.iter().map(|num| num.to_string()).collect();

    format!(
        "{}: .word {}, {}\n.byte {}",
        name,
        dimensions.0,
        dimensions.1,
        string_image_array.join(",")
    )
}

#[derive(Parser)]
struct Cli {
    image_path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let image: DynamicImage = image::open(&args.image_path).unwrap();

    let name = args.image_path.file_stem().unwrap().to_str().unwrap();
    println!("name: {}", name);

    // The dimensions method returns the images width and height.
    let dimensions: (u32, u32) = image.dimensions();
    println!("dimensions {:?}", dimensions);

    let image_array: Vec<u8> = image_to_riscv(image, dimensions);
    println!("image length {:?}", image_array.len());

    let data = image_array_to_string(name, image_array, dimensions);
    fs::write(format!("./{}.s", name), data).expect("Unable to write file");
}
