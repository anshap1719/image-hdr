//! An implementation of HDR merging via "Poisson Photon Noise Estimator" as introduced in
//! [Noise-Aware Merging of High Dynamic Range Image Stacks without Camera Calibration](https://www.cl.cam.ac.uk/research/rainbow/projects/noise-aware-merging/2020-ppne-mle.pdf)

use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::error::UnknownError;
use crate::input::HDRInput;
use crate::Error;

const RED_COEFFICIENT: f32 = 1.;
const GREEN_COEFFICIENT: f32 = 1.;
const BLUE_COEFFICIENT: f32 = 1.;

/// Calculate the poisson estimate for an image.
/// Given a set of image paths, this returns a
/// pixel buffer of the resultant HDR merge of
/// supplied images.
///
/// For more details on the algorithm used, please
/// refer to [Noise-Aware Merging of High Dynamic Range Image Stacks without Camera Calibration](https://www.cl.cam.ac.uk/research/rainbow/projects/noise-aware-merging/2020-ppne-mle.pdf)
///
/// specifically the section about "Poisson Photon Noise Estimator"
///
/// # Errors
/// If supplied image is not an RGB image. Non RGB images
/// include images with alpha channel, grayscale images,
/// and images with other color encodings (like CMYK).
pub(crate) fn calculate_poisson_estimate(inputs: &[HDRInput]) -> Result<Vec<f32>, Error> {
    let radiances: Vec<Vec<f32>> = inputs
        .par_iter()
        .map(|input| {
            let pixels = input.get_image().to_rgb32f().into_raw();
            let scaled_radiances: Vec<f32> = pixels
                .chunks_exact(3)
                .flat_map(|channels| {
                    if let [r, g, b] = channels {
                        let scaling_factor = input.get_exposure() * input.get_gain();

                        [
                            r / (scaling_factor * RED_COEFFICIENT),
                            g / (scaling_factor * GREEN_COEFFICIENT),
                            b / (scaling_factor * BLUE_COEFFICIENT),
                        ]
                    } else {
                        panic!("Invalid channels");
                    }
                })
                .collect();

            scaled_radiances
        })
        .collect();

    let sum_exposures: f32 = inputs.iter().map(HDRInput::get_exposure).sum();

    let phi: Vec<f32> = radiances.iter().enumerate().fold(
        radiances
            .first()
            .ok_or(Error::UnknownError(UnknownError::from(
                "Invalid radiances".to_string(),
            )))?
            .clone(),
        |acc, (index, radiances)| {
            acc.par_iter()
                .zip(radiances)
                .map(|(previous, current)| {
                    ((previous + current) * inputs.get(index).unwrap().get_exposure())
                        / sum_exposures
                })
                .collect()
        },
    );

    Ok(phi)
}
