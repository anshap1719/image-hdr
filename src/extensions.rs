//! Extensions on top of dependencies to facilitate the implementations of this library

use image::{DynamicImage, ImageBuffer, Luma, Rgb};
use ndarray::Array3;

/// Trait to add the ability to get a nd-array buffer from the target type
pub trait NDArrayBuffer {
    /// Get the buffer as `Array3<f32>`
    fn to_nd_array_buffer(&self) -> Array3<f32>;

    /// Generate a new instance of the target from a nd-array buffer.
    fn from_nd_array_buffer(buffer: Array3<f32>) -> Self;
}

impl NDArrayBuffer for DynamicImage {
    fn to_nd_array_buffer(&self) -> Array3<f32> {
        match self {
            DynamicImage::ImageLuma8(_)
            | DynamicImage::ImageLumaA8(_)
            | DynamicImage::ImageLuma16(_)
            | DynamicImage::ImageLumaA16(_) => {
                let mut buffer =
                    Array3::<f32>::zeros((self.height() as usize, self.width() as usize, 1));

                for (x, y, pixel) in self.to_luma32f().enumerate_pixels() {
                    buffer[[y as usize, x as usize, 0]] = pixel.0[0];
                }

                buffer
            }
            _ => {
                let mut buffer =
                    Array3::<f32>::zeros((self.height() as usize, self.width() as usize, 3));

                for (x, y, pixel) in self.to_rgb32f().enumerate_pixels() {
                    let [red, green, blue] = pixel.0;

                    buffer[[y as usize, x as usize, 0]] = red;
                    buffer[[y as usize, x as usize, 1]] = green;
                    buffer[[y as usize, x as usize, 2]] = blue;
                }

                buffer
            }
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn from_nd_array_buffer(buffer: Array3<f32>) -> Self {
        if let (height, width, 1) = buffer.dim() {
            let mut result = ImageBuffer::<Luma<u16>, Vec<u16>>::new(width as u32, height as u32);
            for (x, y, pixel) in result.enumerate_pixels_mut() {
                let intensity = buffer[[y as usize, x as usize, 0]] * f32::from(u16::MAX);
                *pixel = Luma([intensity as u16]);
            }

            DynamicImage::ImageLuma16(result)
        } else if let (height, width, 3) = buffer.dim() {
            let mut result = ImageBuffer::<Rgb<f32>, Vec<f32>>::new(width as u32, height as u32);
            for (x, y, pixel) in result.enumerate_pixels_mut() {
                let red = buffer[[y as usize, x as usize, 0]];
                let green = buffer[[y as usize, x as usize, 1]];
                let blue = buffer[[y as usize, x as usize, 2]];

                *pixel = Rgb([red, green, blue]);
            }

            DynamicImage::ImageRgb32F(result)
        } else {
            panic!("Unexpected dimensions encountered");
        }
    }
}
