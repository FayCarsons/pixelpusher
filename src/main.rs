#![feature(slice_take)]
mod utils;
use image::{save_buffer_with_format, EncodableLayout, GenericImageView, ImageFormat};
use rayon::{
    prelude::{IntoParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use std::time::Instant;
use utils::{SortMode, Direction, CONFIG, hue_compare, luma_compare};

fn main() {
    let path = &CONFIG.input_path;
    let start = Instant::now();

    let img = image::open(path).unwrap();
    let (width, height) = img.dimensions();

    let rgb = img.to_rgb8();
    let bytes = rgb.as_bytes();
    let mut pixels: Vec<&[u8]> = bytes.chunks(3).collect();

    let compare_fn = match CONFIG.mode {
        SortMode::Luma => luma_compare,
        SortMode::Hue => hue_compare
    };

    match CONFIG.direction {
        Direction::Horizontal => {
            pixels
                .par_chunks_mut(width as usize)
                .for_each(|row| row.par_sort_by(compare_fn));
        }
        Direction::Vertical => {
            let mut columns = (0..width as usize)
                .into_par_iter()
                .map(|x| {
                    (0..height as usize)
                        .into_iter()
                        .map(|y| pixels[x + (y * width as usize)])
                        .collect::<Vec<&[u8]>>()
                })
                .collect::<Vec<Vec<&[u8]>>>();
            columns
                .iter_mut()
                .for_each(|col| col.par_sort_by(luma_compare));
            pixels = (0..width * height)
                .map(|i| columns[i as usize % width as usize][i as usize / width as usize])
                .collect();
        }
    };

    let flat_buffer = pixels
        .into_iter()
        .flatten()
        .map(|x| *x)
        .collect::<Vec<u8>>();

    save_buffer_with_format(
        &(CONFIG.output_path.to_owned() + ".bmp"),
        &flat_buffer,
        width,
        height,
        image::ColorType::Rgb8,
        ImageFormat::Bmp,
    )
    .expect("unable to save image");

    println!("elapsed time {:?}", start.elapsed());
}
