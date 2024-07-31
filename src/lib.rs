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

use image::DynamicImage;
use poisson::calculate_poisson_estimate;

pub mod error;
pub mod exif;
pub mod extensions;
pub mod input;
mod io;
mod poisson;
pub mod stretch;

use crate::extensions::NDArrayBuffer;
use crate::input::HDRInputList;
pub use error::Error;

/// Given a set of file paths, attempt to HDR merge the images
/// and produce a single [`DynamicImage`] (from image-rs crate).
///
/// # Errors
/// - If image list is empty
/// - If supplied image is not an RGB image. Non RGB images include images with alpha channel, grayscale images, and images with other color encodings (like CMYK).
/// - If images are of different dimensions.
pub fn hdr_merge_images(inputs: &mut HDRInputList) -> Result<DynamicImage, Error> {
    if inputs.len() < 2 {
        return Err(Error::InputError {
            parameter_name: "paths".to_string(),
            message: "At least two images must be provided".to_string(),
        });
    }

    let phi = calculate_poisson_estimate(inputs.as_slice_mut());

    Ok(DynamicImage::from_nd_array_buffer(phi))
}
