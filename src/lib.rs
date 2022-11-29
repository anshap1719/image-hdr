use image::{DynamicImage, GenericImageView, ImageBuffer};
use io::read_image;
use poisson::calculate_poisson_estimate;

mod exif;
mod io;
mod poisson;
pub mod stretch;
mod util;

/// Given a set of file paths, attempt to HDR merge the images
/// and produce a single DynamicImage (from image-rs crate).
///
/// # Panics
/// - If image list is empty
/// - If supplied image is not an RGB image. Non RGB images include images with alpha channel, grayscale images, and images with other color encodings (like CMYK).
/// - If images are of different dimensions.
pub fn hdr_merge_images(paths: Vec<String>) -> DynamicImage {
    let image = read_image(&paths.first().unwrap());
    let (width, height) = image.dimensions();

    drop(image);

    let phi = calculate_poisson_estimate(&paths);

    DynamicImage::ImageRgb32F(ImageBuffer::from_vec(width, height, phi.to_vec()).unwrap())
}
