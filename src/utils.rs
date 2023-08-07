use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SortMode {
    Luma,
    Hue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub direction: Direction,
    pub mode: SortMode,
    pub low_threshold: u8,
    pub high_threshold: u8,
    pub input_path: String,
    pub output_path: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let mut file = File::open("config.ron").expect("cannot open config");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("cannot read config");
    ron::from_str(&buffer).expect("cannot parse config")
});

/*
pub const IN_PATH: &str = "./resources/smaller.png";
pub const OUT_NAME: &str = "test";
pub const MODE: Mode = Mode::Vertical;
const LOW_THRESH: u8 = 32;
const HI_THRESH: u8 = 128;
*/

fn in_range(luma_one: u8, luma_two: u8) -> bool {
    !(luma_one < CONFIG.low_threshold
        || luma_one > CONFIG.high_threshold
        || luma_two < CONFIG.low_threshold
        || luma_two > CONFIG.high_threshold)
}

fn luma(pixel: &[u8]) -> u8 {
    (pixel.iter().map(|x| *x as u16).sum::<u16>() / 3) as u8
}

pub fn luma_compare(pixel_one: &&[u8], pixel_two: &&[u8]) -> Ordering {
    let first = luma(pixel_one);
    let second = luma(pixel_two);
    if in_range(first, second) {
        first.cmp(&second)
    } else {
        Ordering::Equal
    }
}

fn hue(pixel: &[u8]) -> u8 {
    let floats = pixel
        .iter()
        .map(|c| *c as f32 / 255f32)
        .collect::<Vec<f32>>();
    let max = *pixel.iter().max().unwrap() as f32 / 255.;
    let min = *pixel.iter().min().unwrap() as f32 / 255.;
    let diff = max - min;

    let h = if let Some(idx) = pixel
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(idx, _)| idx)
    {
        match idx {
            0 => (floats[1] - floats[2]) / diff,
            1 => 2. + (floats[2] - floats[0]) / diff,
            2 => 4. + (floats[0] - floats[1]) / diff,
            _ => 0.,
        }
    } else {
        180.
    };

    let h = h.abs() * 60.;

    ((h / 360.) * u8::MAX as f32) as u8
}

pub fn hue_compare(pixel_one: &&[u8], pixel_two: &&[u8]) -> Ordering {
    let first = luma(pixel_one);
    let second = luma(pixel_two);
    if in_range(first, second) {
        hue(&pixel_one).cmp(&hue(&pixel_two))
    } else {
        Ordering::Equal
    }
}
