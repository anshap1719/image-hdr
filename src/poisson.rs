use image::DynamicImage;
use rayon::{
    prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSlice,
};

use crate::{
    exif::{get_exif_data, get_exposures, get_gains},
    io::read_image,
};

const RED_COEFFICIENT: f32 = 1.;
const GREEN_COEFFICIENT: f32 = 1.;
const BLUE_COEFFICIENT: f32 = 1.;

/// Calculate the poisson estimate for an image.
/// Given a set of image paths, this returns a
/// pixel buffer of the resultant HDR merge of
/// supplied images.
///
/// For more details on the algorithm used, please
/// refer to https://www.cl.cam.ac.uk/research/rainbow/projects/noise-aware-merging/2020-ppne-mle.pdf
/// specifically the section about "Poisson Photon Noise Estimator"
///
/// # Panics
/// If supplied image is not an RGB image. Non RGB images
/// include images with alpha channel, grayscale images,
/// and images with other color encodings (like CMYK).
pub(crate) fn calculate_poisson_estimate(paths: &[String]) -> Vec<f32> {
    let exif = get_exif_data(paths);
    let exposures = get_exposures(&exif);
    let gains = get_gains(&exif);

    let images: Vec<DynamicImage> = paths.par_iter().map(read_image).collect();

    let radiances: Vec<Vec<f32>> = images
        .par_iter()
        .zip(&exposures)
        .zip(gains)
        .map(|((image, exposure), gain)| {
            let pixels = image.to_rgb32f().into_raw();
            let scaled_radiances: Vec<f32> = pixels
                .par_chunks_exact(3)
                .flat_map(|channels| {
                    if let [r, g, b] = channels {
                        let scaling_factor = exposure * gain;

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

    let sum_exposures: f32 = exposures.iter().sum();

    let phi: Vec<f32> = radiances.iter().enumerate().fold(
        radiances.first().unwrap().to_vec(),
        |acc, (index, radiances)| {
            acc.par_iter()
                .zip(radiances)
                .map(|(previous, current)| (previous + current) * exposures[index])
                .map(|item| item / sum_exposures)
                .collect()
        },
    );

    phi
}
