use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, Rgba};
use std::{fs::File, io::BufReader, io::Write, path::Path};

use crate::braille;

const CHUNK_WIDTH: usize = 2;
const CHUNK_HEIGHT: usize = 4;

type RgbaPixelChunk = [[Rgba<u8>; CHUNK_WIDTH]; CHUNK_HEIGHT];
type BoolPixelChunk = [[bool; CHUNK_WIDTH]; CHUNK_HEIGHT];

pub struct ImageToBrailleConverter {
    pub image: DynamicImage,
    pub width: usize,
    pub height: usize,
}

impl ImageToBrailleConverter {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let img = ImageReader::new(reader).with_guessed_format()?.decode()?;

        let (width, height) = img.dimensions();

        Ok(Self {
            image: img,
            width: width as usize,
            height: height as usize,
        })
    }

    pub fn resize(&self, desired_width_chars: usize) -> Self {
        let desired_width = desired_width_chars * CHUNK_WIDTH;
        let scaled_height =
            (desired_width as f32 / self.width as f32 * self.height as f32) as usize;
        let cropped_height = scaled_height - (scaled_height % CHUNK_HEIGHT);

        let scaled_img = self.image.resize_exact(
            desired_width as u32,
            cropped_height as u32,
            image::imageops::FilterType::Nearest,
        );

        Self {
            image: scaled_img,
            width: desired_width,
            height: cropped_height,
        }
    }

    pub fn convert(&self, output: &mut impl Write) -> Result<(), Box<dyn std::error::Error>> {
        let rgb_chunks = chunkify_image(&self.image);
        let bool_chunks = filter_chunks(rgb_chunks, brightness_filter);

        for lines in bool_chunks.chunks(self.width / CHUNK_WIDTH) {
            for chunk in lines {
                let braille_char = braille::chunk_to_braille(chunk.clone());
                write!(output, "{}", braille_char)?;
            }
            writeln!(output)?;
        }

        Ok(())
    }
}

fn brightness_filter(pixel: Rgba<u8>) -> bool {
    let Rgba([r, g, b, _]) = pixel;
    let brightness = (r as f32 + g as f32 + b as f32) / 3.0;

    brightness < 128.0
}

fn chunkify_image(img: &image::DynamicImage) -> Vec<RgbaPixelChunk> {
    let (width, height) = img.dimensions();

    let mut chunks = Vec::new();

    for y in (0..height).step_by(CHUNK_HEIGHT) {
        for x in (0..width).step_by(CHUNK_WIDTH) {
            let mut chunk = [[Rgba([0, 0, 0, 0]); CHUNK_WIDTH]; CHUNK_HEIGHT];

            for r in 0..CHUNK_HEIGHT {
                for c in 0..CHUNK_WIDTH {
                    chunk[r][c] = img.get_pixel(x + c as u32, y + r as u32);
                }
            }

            chunks.push(chunk);
        }
    }

    chunks
}

fn filter_chunks(
    chunks: Vec<RgbaPixelChunk>,
    predicate: impl Fn(Rgba<u8>) -> bool,
) -> Vec<BoolPixelChunk> {
    chunks
        .into_iter()
        .map(|chunk| filter_chunk(chunk, &predicate))
        .collect()
}

fn filter_chunk(chunk: RgbaPixelChunk, predicate: &impl Fn(Rgba<u8>) -> bool) -> BoolPixelChunk {
    let mut out = [[false; CHUNK_WIDTH]; CHUNK_HEIGHT];

    for r in 0..CHUNK_HEIGHT {
        for c in 0..CHUNK_WIDTH {
            out[r][c] = predicate(chunk[r][c]);
        }
    }

    out
}
