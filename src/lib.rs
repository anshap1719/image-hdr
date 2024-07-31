//! An implementation of HDR Radiance Estimation using Poisson Photon Noise Estimator for creating HDR image from a set of images

#![deny(clippy::correctness)]
#![deny(clippy::suspicious)]
#![deny(clippy::complexity)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]
#![warn(missing_docs)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::module_name_repetitions)]

use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Rgb};
use poisson::calculate_poisson_estimate;

pub mod error;
pub mod exif;
pub mod input;
mod io;
mod poisson;
pub mod stretch;
mod util;

use crate::error::UnknownError;
use crate::input::HDRInputList;
pub use error::Error;

/// Given a set of file paths, attempt to HDR merge the images
/// and produce a single [`DynamicImage`] (from image-rs crate).
///
/// # Errors
/// - If image list is empty
/// - If supplied image is not an RGB image. Non RGB images include images with alpha channel, grayscale images, and images with other color encodings (like CMYK).
/// - If images are of different dimensions.
pub fn hdr_merge_images(mut inputs: HDRInputList) -> Result<DynamicImage, Error> {
    if inputs.len() < 2 {
        return Err(Error::InputError {
            parameter_name: "paths".to_string(),
            message: "At least two images must be provided".to_string(),
        });
    }

    let phi = calculate_poisson_estimate(inputs.as_slice_mut());
    let dimensions = phi.dim();

    let output = if let (height, width, 1) = dimensions {
        let mut result = ImageBuffer::<Luma<u16>, Vec<u16>>::new(width as u32, height as u32);
        for (x, y, pixel) in result.enumerate_pixels_mut() {
            let intensity = phi[[y as usize, x as usize, 0]] * u16::MAX as f32;
            *pixel = Luma([intensity as u16]);
        }

        DynamicImage::ImageLuma16(result)
    } else if let (height, width, 3) = dimensions {
        let mut result = ImageBuffer::<Rgb<f32>, Vec<f32>>::new(width as u32, height as u32);
        for (x, y, pixel) in result.enumerate_pixels_mut() {
            let red = phi[[y as usize, x as usize, 0]];
            let green = phi[[y as usize, x as usize, 1]];
            let blue = phi[[y as usize, x as usize, 2]];

            *pixel = Rgb([red, green, blue]);
        }

        DynamicImage::ImageRgb32F(result)
    } else {
        panic!("Unexpected dimensions encountered");
    };

    Ok(output)
}
