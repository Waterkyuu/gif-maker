use anyhow::{anyhow, Result};
use gif::{Encoder, Frame, Repeat};
use image::ImageReader;
use std::fs::File;
use std::path::PathBuf;

pub fn create_gif(
    image_paths: Vec<PathBuf>,
    output_path: &str,
    fps: u16,
) -> Result<()> {
    if image_paths.is_empty() {
        return Err(anyhow!("No images found"));
    }

    // Open the first image for decoding
    let first_image = ImageReader::open(&image_paths[0])?.decode()?.to_rgb8();

    let width = first_image.width() as u16;
    let height = first_image.height() as u16;

    let output_file = File::create(output_path)?;


    // Create a GIF encoder
    // output_file: GIF The file that needs to be written
    // &[]: Global color palette, the palette is not manually specified here.
    let mut encoder = Encoder::new(output_file, width, height, &[])?;

    encoder.set_repeat(Repeat::Infinite)?;

    let delay = (100 / fps.max(1)).max(1);

    //   Image file
    //   -> Decode into an RGBA image
    //   -> Convert to a `Vec<u8>` pixel array using `into_raw()`
    //   -> Convert to a GIF frame using `Frame::from_rgba_speed(...)`
    //   -> Write to the GIF file using `encoder.write_frame(...)`
    for path in image_paths {
        let img = ImageReader::open(&path)?
            .decode()?
            .resize_exact(
                width as u32,
                height as u32,
                image::imageops::FilterType::Lanczos3,
            )
            .to_rgba8();

        let mut pixels = img.into_raw();

        let mut frame = Frame::from_rgba_speed(
            width,
            height,
            &mut pixels,
            10,
        );

        frame.delay = delay;

        encoder.write_frame(&frame)?;
    }

    Ok(())
}
