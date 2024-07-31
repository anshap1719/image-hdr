//! An implementation of HDR merging via "Poisson Photon Noise Estimator" as introduced in
//! [Noise-Aware Merging of High Dynamic Range Image Stacks without Camera Calibration](https://www.cl.cam.ac.uk/research/rainbow/projects/noise-aware-merging/2020-ppne-mle.pdf)

use crate::input::HDRInput;
use ndarray::array;
use ndarray::prelude::*;
use rayon::prelude::*;

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
pub(crate) fn calculate_poisson_estimate(inputs: &mut [HDRInput]) -> Array3<f32> {
    inputs.par_iter_mut().for_each(|input| {
        let scaling_factor = input.get_exposure() * input.get_gain();
        let input_buffer = input.get_buffer_mut();

        if let (_, _, 1) = input_buffer.dim() {
            *input_buffer /= scaling_factor;
        } else if let (_, _, 3) = input_buffer.dim() {
            *input_buffer /= &array![[[
                scaling_factor * RED_COEFFICIENT,
                scaling_factor * GREEN_COEFFICIENT,
                scaling_factor * BLUE_COEFFICIENT
            ]]];
        } else {
            panic!("Unexpected scaling matrix encountered.")
        }
    });

    let shape = inputs.first().unwrap().get_buffer().dim();
    let sum_exposures: f32 = inputs.iter().map(HDRInput::get_exposure).sum();

    let normalized_radiances = inputs
        .par_iter()
        .map(|input| {
            let mut radiance = input.get_buffer().clone();
            let exposure = input.get_exposure();

            radiance *= exposure / sum_exposures;

            radiance
        })
        .reduce(
            || Array3::<f32>::zeros(shape),
            |acc, radiance| acc + radiance,
        );

    normalized_radiances
}
