use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, Rgba};
use std::{fs::File, io::BufReader, io::Write, path::Path};

use crate::braille;

const CHUNK_WIDTH: u32 = 2;
const CHUNK_HEIGHT: u32 = 4;

type RgbaPixelChunk = [[Rgba<u8>; CHUNK_WIDTH as usize]; CHUNK_HEIGHT as usize];
type BoolPixelChunk = [[bool; CHUNK_WIDTH as usize]; CHUNK_HEIGHT as usize];

pub struct ImageToBrailleConverter {
    pub image: DynamicImage,
    pub width: u32,
    pub height: u32,
}

impl ImageToBrailleConverter {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let img = ImageReader::new(reader).with_guessed_format()?.decode()?;

        let (width, height) = img.dimensions();

        Ok(Self {
            image: img,
            width,
            height,
        })
    }

    pub fn resize(&self, desired_width_chars: u32) -> Self {
        let desired_width = desired_width_chars * CHUNK_WIDTH;
        let scaled_height = (desired_width as f32 / self.width as f32 * self.height as f32) as u32;
        let cropped_height = scaled_height - (scaled_height % CHUNK_HEIGHT);

        let scaled_img = self.image.resize_exact(
            desired_width,
            cropped_height,
            image::imageops::FilterType::Nearest,
        );

        Self {
            image: scaled_img,
            width: desired_width,
            height: cropped_height,
        }
    }

    pub fn convert(&self, output: &mut impl Write) -> Result<(), Box<dyn std::error::Error>> {
        let chunks = chunkify_image(&self.image);
        let chunks = filter_chunks(chunks, |p| {
            (p[0] as u16 + p[1] as u16 + p[2] as u16) / 3 < ((255 as f64 * 0.8) as u16)
        });

        for lines in chunks.chunks(self.width as usize / CHUNK_WIDTH as usize) {
            for chunk in lines {
                let braille_char = braille::chunk_to_braille(chunk.clone());
                write!(output, "{}", braille_char)?;
            }
            writeln!(output)?;
        }

        Ok(())
    }
}

fn chunkify_image(img: &image::DynamicImage) -> Vec<RgbaPixelChunk> {
    let (width, height) = img.dimensions();

    let mut chunks = Vec::new();

    for y in (0..height).step_by(CHUNK_HEIGHT as usize) {
        for x in (0..width).step_by(CHUNK_WIDTH as usize) {
            let mut chunk = [[Rgba([0, 0, 0, 0]); CHUNK_WIDTH as usize]; CHUNK_HEIGHT as usize];

            for r in 0..(CHUNK_HEIGHT as usize) {
                for c in 0..(CHUNK_WIDTH as usize) {
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
    let mut out = [[false; CHUNK_WIDTH as usize]; CHUNK_HEIGHT as usize];

    for r in 0..(CHUNK_HEIGHT as usize) {
        for c in 0..(CHUNK_WIDTH as usize) {
            out[r][c] = predicate(chunk[r][c]);
        }
    }

    out
}
