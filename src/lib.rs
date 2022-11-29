use image::{DynamicImage, GenericImageView, ImageBuffer};
use io::read_image;
use poisson::calculate_poisson_estimate;

mod exif;
mod io;
mod poisson;
pub mod stretch;
mod util;

pub fn hdr_merge_images(paths: Vec<String>) -> DynamicImage {
    let image = read_image(&paths.first().unwrap());
    let (width, height) = image.dimensions();

    drop(image);

    let phi = calculate_poisson_estimate(&paths);

    DynamicImage::ImageRgb32F(ImageBuffer::from_vec(width, height, phi.to_vec()).unwrap())
}
