use ::image::{ImageBuffer, Rgb, DynamicImage};
use imagepipe::{ImageSource, Pipeline};

pub(crate) fn read_image(path: &String) -> DynamicImage {
    let image = image::open(path);

     match image {
        Ok(image) => {
            image
        }
        Err(_) => {
            read_raw_image(path).expect("Failed to load image: Unexpected format encountered")
        }
    }
}

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
