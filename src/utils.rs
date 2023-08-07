use std::cmp::Ordering;

pub enum Mode {
    Horizontal,
    Vertical
}

pub const IN_PATH: &str  = "test.jpg";
pub const OUT_NAME: &str  = "fmt_test";
pub const MODE: Mode = Mode::Vertical;
const LOW_THRESH: u8 = 32;
const HI_THRESH: u8 = 128;

fn in_range(luma_one: u8, luma_two: u8) -> bool {
   ! ( luma_one < LOW_THRESH || luma_one > HI_THRESH || luma_two < LOW_THRESH || luma_two > HI_THRESH)
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

/* fn hue(pixel: &[u8; 3]) -> u8 {
    let floats = pixel.iter().map(|c| *c as f32 / 255f32).collect::<Vec<f32>>();
    let diff = floats.iter().reduce(|a,b| f32::max(*a,b)).unwrap() - floats.iter().reduce(|a,b| &a.min(*b)).unwrap();
    
    let h = if let Some(idx) = pixel.iter().enumerate().max_by(|(_, a), (_, b)| a.cmp(b)).map(|(idx,_)| idx) {
        match idx {
            0 => (floats[1] - floats[2]) / diff,
            1 => 2. + (floats[2] - floats[0]) / diff,
            2 => 4. + (floats[0] - floats[1]) / diff,
            _ => 0.
        }
    } else {
        160.
    };

    let h = h.abs() * 60.;

    (h * u8::MAX as f32) as u8
} */