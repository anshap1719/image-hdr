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

use image::{DynamicImage, GenericImageView, ImageBuffer};
use io::read_image;
use poisson::calculate_poisson_estimate;

pub mod error;
mod exif;
mod io;
mod poisson;
pub mod stretch;
mod util;
use crate::error::UnknownError;
pub use error::Error;

/// Given a set of file paths, attempt to HDR merge the images
/// and produce a single [`DynamicImage`] (from image-rs crate).
///
/// # Errors
/// - If image list is empty
/// - If supplied image is not an RGB image. Non RGB images include images with alpha channel, grayscale images, and images with other color encodings (like CMYK).
/// - If images are of different dimensions.
pub fn hdr_merge_images(paths: &[String]) -> Result<DynamicImage, Error> {
    let first_image = paths.first().ok_or(Error::InputError {
        parameter_name: "paths".to_string(),
        message: "At least two images must be provided".to_string(),
    })?;

    if paths.len() < 2 {
        return Err(Error::InputError {
            parameter_name: "paths".to_string(),
            message: "At least two images must be provided".to_string(),
        });
    }

    let image = read_image(first_image)?;
    let (width, height) = image.dimensions();

    drop(image);

    let phi = calculate_poisson_estimate(paths)?;

    Ok(DynamicImage::ImageRgb32F(
        ImageBuffer::from_vec(width, height, phi.clone()).ok_or(Error::UnknownError(
            UnknownError::from(
                "Failed to create image buffer, buffer is not large enough".to_string(),
            ),
        ))?,
    ))
}
