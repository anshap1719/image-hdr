//! Apply basic histogram stretch to a linear image to make it viewable.

use crate::Error;
use crate::UnknownError;
use image::{DynamicImage, GenericImageView, ImageBuffer};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::util::transpose_vec;

fn scale_pixel(pixel: f32, min: f32, max: f32) -> f32 {
    (pixel - min) * (1. / (max - min))
}

/// Contrast stretch (normalize) a given image.
///
/// # Errors
/// - if image cannot be constructed from processed pixels.
pub fn apply_histogram_stretch(image: &DynamicImage) -> Result<DynamicImage, Error> {
    let (width, height) = image.dimensions();

    let image_buffer = image.to_rgb32f();
    let image_buffer = image_buffer.as_raw();

    let pixel_wise_channels = image_buffer
        .chunks_exact(3)
        .map(|chunk| -> Vec<f32> { chunk.to_vec() })
        .collect();

    let channel_wise_pixels = transpose_vec(&pixel_wise_channels);

    let channel_wise_pixels: Vec<Vec<f32>> = channel_wise_pixels
        .par_iter()
        .map(|channel| {
            let input_max_value = channel.iter().copied().reduce(f32::max).unwrap_or(1.);
            let input_min_value = channel.iter().copied().reduce(f32::min).unwrap_or(0.);

            let pixels: Vec<f32> = channel
                .iter()
                .map(|pixel| scale_pixel(*pixel, input_min_value, input_max_value))
                .collect();

            pixels
        })
        .collect();

    let pixels_buf: Vec<f32> = transpose_vec(&channel_wise_pixels)
        .iter()
        .flatten()
        .copied()
        .collect();

    Ok(DynamicImage::ImageRgb32F(
        ImageBuffer::from_vec(width, height, pixels_buf).ok_or(Error::UnknownError(
            UnknownError::from(
                "Failed to create image buffer, buffer is not large enough".to_string(),
            ),
        ))?,
    ))
}
