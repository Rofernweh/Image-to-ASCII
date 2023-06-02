use image::{imageops::FilterType, GenericImageView, GrayImage, ImageBuffer, Luma};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide an image file");
        std::process::exit(1);
    }

    let img = image::open(&args[1]).unwrap().to_luma8();
    let scaled = image::imageops::resize(
        &img,
        img.width() / 10,
        img.height() / 20,
        FilterType::Nearest,
    );

    for y in 0..scaled.height() {
        for x in 0..scaled.width() {
            let pixel = scaled.get_pixel(x, y);
            print!("{}", to_ascii(pixel));
        }
        println!();
    }
}

fn to_ascii(pixel: &Luma<u8>) -> char {
    let ascii_chars = "$@B%8WM#*oahkbdpqwmZO0QCJYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
    let ascii_scale = ascii_chars.len() - 1;
    let brightness = pixel.0[0] as f32 / 255.0;
    let index = (brightness * ascii_scale as f32) as usize;
    ascii_chars.chars().nth(index).unwrap()
}
