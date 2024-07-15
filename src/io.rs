//! Helper functions to read and decode images

use crate::Error;
use image::DynamicImage;

/// Given a path to a file, attempt to read the image.
/// The function supports reading raw images. All
/// formats and cameras supported by rawloader crate
/// [rawloader](https://github.com/pedrocr/rawloader) are supported.
///
/// # Errors
/// If image cannot be read
pub(crate) fn read_image(path: &String) -> Result<DynamicImage, Error> {
    match image::open(path) {
        Ok(image) => Ok(image),
        Err(_err) => {
            #[cfg(not(feature = "read-raw-image"))]
            return Err(_err.into());
            #[cfg(feature = "read-raw-image")]
            Ok(read_raw_image(path)?)
        }
    }
}

/// Given a path to a file, attempt to read the RAW image.
/// All formats and cameras supported by rawloader crate
/// [rawloader](https://github.com/pedrocr/rawloader) are supported.
#[cfg(feature = "read-raw-image")]
pub(crate) fn read_raw_image(path: &String) -> Result<DynamicImage, Error> {
    use crate::error::{RawPipelineError, UnknownError};
    use image::{ImageBuffer, Rgb};
    use imagepipe::{ImageSource, Pipeline};

    let raw = rawloader::decode_file(path)?;

    let source = ImageSource::Raw(raw);
    let mut pipeline = Pipeline::new_from_source(source).map_err(RawPipelineError::from)?;

    pipeline.run(None);

    let image = pipeline
        .output_16bit(None)
        .map_err(RawPipelineError::from)?;
    let image = ImageBuffer::<Rgb<u16>, Vec<u16>>::from_raw(
        u32::try_from(image.width).map_err(|err| UnknownError::from(err.to_string()))?,
        u32::try_from(image.height).map_err(|err| UnknownError::from(err.to_string()))?,
        image.data,
    );

    match image {
        Some(image) => Ok(DynamicImage::ImageRgb16(image)),
        None => Err(Error::RawPipeline(RawPipelineError::from(
            "Failed to load pipeline output".to_string(),
        ))),
    }
}
