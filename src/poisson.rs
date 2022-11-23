use image::DynamicImage;
use rayon::{prelude::{IntoParallelRefIterator, ParallelIterator, IndexedParallelIterator}, slice::ParallelSlice};

use crate::{exif::{get_exposures, get_gains, get_exif_data}, io::read_image};

const RED_COEFFICIENT: f32 = 1.;
const GREEN_COEFFICIENT: f32 = 1.;
const BLUE_COEFFICIENT: f32 = 1.;

pub(crate) fn calculate_poisson_estimate(paths: &[String]) -> Vec<f32> {
    let exif = get_exif_data(paths);
    let exposures = get_exposures(&exif);
    let gains = get_gains(&exif);

    println!("{:?}, {:?}", exposures, gains);

    let images: Vec<DynamicImage> = paths.par_iter().map(read_image).collect();

    let radiances: Vec<Vec<f32>> = images.par_iter()
        .zip(&exposures).zip(gains).map(|((image, exposure), gain)| {
            let pixels = image.to_rgb32f().into_raw();
            let scaled_radiances: Vec<f32> = pixels
                .par_chunks_exact(3)
                .flat_map(|channels| {
                    if let [r, g, b] = channels {
                        let scaling_factor = exposure * gain;

                        [
                            r / (scaling_factor * RED_COEFFICIENT),
                            g / (scaling_factor * GREEN_COEFFICIENT),
                            b / (scaling_factor * BLUE_COEFFICIENT)
                        ]
                    } else {
                        panic!("Invalid channels");
                    }
                }).collect();

            scaled_radiances
        }).collect();

    let sum_exposures: f32 = exposures.iter().sum();

    let phi: Vec<f32> = radiances.iter().enumerate().fold(radiances.first().unwrap().to_vec(), |acc, (index, radiances)| {
        acc.par_iter().zip(radiances).map(|(previous, current)| {
            (previous + current) * exposures[index]
        }).map(|item| item / sum_exposures).collect()
    });

    phi
}
