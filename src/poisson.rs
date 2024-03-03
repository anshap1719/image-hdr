//! An implementation of HDR merging via "Poisson Photon Noise Estimator" as introduced in
//! [Noise-Aware Merging of High Dynamic Range Image Stacks without Camera Calibration](https://www.cl.cam.ac.uk/research/rainbow/projects/noise-aware-merging/2020-ppne-mle.pdf)

use image::{DynamicImage, Pixel};
use rayon::prelude::*;

use crate::error::UnknownError;
use crate::input::HDRInput;
use crate::Error;
use num_traits::cast::ToPrimitive;
use num_traits::{Bounded, NumCast};

const RED_COEFFICIENT: f32 = 1.;
const GREEN_COEFFICIENT: f32 = 1.;
const BLUE_COEFFICIENT: f32 = 1.;

trait LinearScalablePixel
where
    Self: Pixel,
{
    fn scale_subpixel(subpixel: &mut Self::Subpixel, scale_factor: f32) {
        let linear_pixel = Self::Subpixel::to_f32(subpixel).unwrap() / f32::MAX;
        let scaled_pixel = linear_pixel * scale_factor;

        *subpixel = <Self::Subpixel as NumCast>::from(
            scaled_pixel * Self::Subpixel::max_value().to_f32().unwrap(),
        )
        .unwrap();
    }

    fn scale(&mut self, scale_factor_r: f32, scale_factor_g: f32, scale_factor_b: f32);
}

impl<T> LinearScalablePixel for T
where
    T: Pixel,
{
    fn scale(&mut self, scale_factor_r: f32, scale_factor_g: f32, scale_factor_b: f32) {
        if let [red, green, blue] = self.channels_mut() {
            Self::scale_subpixel(red, scale_factor_r);
            Self::scale_subpixel(green, scale_factor_g);
            Self::scale_subpixel(blue, scale_factor_b);
        }
    }
}

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
pub(crate) fn calculate_poisson_estimate(inputs: &mut [HDRInput]) -> Result<Vec<f32>, Error> {
    inputs.par_iter_mut().for_each(|input| {
        let scaling_factor = input.get_exposure() * input.get_gain();
        let scaling_factor_r = scaling_factor * RED_COEFFICIENT;
        let scaling_factor_g = scaling_factor * GREEN_COEFFICIENT;
        let scaling_factor_b = scaling_factor * BLUE_COEFFICIENT;

        let image = input.get_image_mut();
        match image {
            DynamicImage::ImageLuma8(_) => {
                unimplemented!()
            }
            DynamicImage::ImageLumaA8(_) => {
                unimplemented!()
            }
            DynamicImage::ImageRgb8(image) => {
                for pixel in image.pixels_mut() {
                    pixel.scale(scaling_factor_r, scaling_factor_g, scaling_factor_b);
                }
            }
            DynamicImage::ImageRgba8(image) => {
                for pixel in image.pixels_mut() {
                    pixel.scale(scaling_factor_r, scaling_factor_g, scaling_factor_b);
                }
            }
            DynamicImage::ImageLuma16(_) => {
                unimplemented!()
            }
            DynamicImage::ImageLumaA16(_) => {
                unimplemented!()
            }
            DynamicImage::ImageRgb16(image) => {
                for pixel in image.pixels_mut() {
                    pixel.scale(scaling_factor_r, scaling_factor_g, scaling_factor_b);
                }
            }
            DynamicImage::ImageRgba16(image) => {
                for pixel in image.pixels_mut() {
                    pixel.scale(scaling_factor_r, scaling_factor_g, scaling_factor_b);
                }
            }
            DynamicImage::ImageRgb32F(image) => {
                for pixel in image.pixels_mut() {
                    pixel.scale(scaling_factor_r, scaling_factor_g, scaling_factor_b);
                }
            }
            DynamicImage::ImageRgba32F(image) => {
                for pixel in image.pixels_mut() {
                    pixel.scale(scaling_factor_r, scaling_factor_g, scaling_factor_b);
                }
            }
            _ => {}
        }
    });

    let sum_exposures: f32 = inputs.iter().map(HDRInput::get_exposure).sum();

    let phi: Vec<f32> = inputs.iter().enumerate().fold(
        inputs
            .first()
            .ok_or(Error::UnknownError(UnknownError::from(
                "Invalid radiances".to_string(),
            )))?
            .get_image()
            .to_rgb32f()
            .into_raw(),
        |acc, (index, radiances)| {
            acc.par_iter()
                .zip(radiances.get_image().to_rgb32f().as_raw())
                .map(|(previous, current)| {
                    ((previous + current) * inputs.get(index).unwrap().get_exposure())
                        / sum_exposures
                })
                .collect()
        },
    );

    Ok(phi)
}
