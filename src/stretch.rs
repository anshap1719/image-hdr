//! Apply basic histogram stretch to a linear image to make it viewable.

use crate::extensions::NDArrayBuffer;
use crate::Error;
use image::DynamicImage;
use rayon::prelude::*;

fn scale_pixel(pixel: f32, min: f32, max: f32) -> f32 {
    (pixel - min) * (1. / (max - min))
}

/// Contrast stretch (normalize) a given image.
///
/// # Errors
/// - if image cannot be constructed from processed pixels.
pub fn apply_histogram_stretch(image: &DynamicImage) -> Result<DynamicImage, Error> {
    let mut buffer = image.to_nd_array_buffer();

    let input_max_value = buffer.iter().copied().reduce(f32::max).unwrap_or(1.);
    let input_min_value = buffer.iter().copied().reduce(f32::min).unwrap_or(0.);

    buffer.par_iter_mut().for_each(|pixel| {
        *pixel = scale_pixel(*pixel, input_min_value, input_max_value);
    });

    Ok(DynamicImage::from_nd_array_buffer(buffer))
}
