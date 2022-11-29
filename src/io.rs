use image::{DynamicImage, ImageBuffer, Rgb};
use imagepipe::{ImageSource, Pipeline};

/// Given a path to a file, attempt to read the image.
/// The function supports reading raw images. All
/// formats and cameras supported by rawloader crate
/// (https://github.com/pedrocr/rawloader) are supported.
///
/// # Panics
/// If image cannot be read
pub(crate) fn read_image(path: &String) -> DynamicImage {
    let image = image::open(path);

    match image {
        Ok(image) => image,
        Err(_) => {
            read_raw_image(path).expect("Failed to load image: Unexpected format encountered")
        }
    }
}

/// Given a path to a file, attempt to read the RAW image.
/// All formats and cameras supported by rawloader crate
/// (https://github.com/pedrocr/rawloader) are supported.
pub(crate) fn read_raw_image(path: &String) -> Option<DynamicImage> {
    let raw = match rawloader::decode_file(path) {
        Ok(raw) => raw,
        Err(_) => return None,
    };

    let source = ImageSource::Raw(raw);
    let mut pipeline = match Pipeline::new_from_source(source) {
        Ok(pipeline) => pipeline,
        Err(_) => return None,
    };

    pipeline.run(None);
    let image = match pipeline.output_16bit(None) {
        Ok(image) => image,
        Err(_) => return None,
    };

    let image = ImageBuffer::<Rgb<u16>, Vec<u16>>::from_raw(
        image.width as u32,
        image.height as u32,
        image.data,
    );

    let image = match image {
        Some(image) => DynamicImage::ImageRgb16(image),
        None => return None,
    };

    Some(image)
}
