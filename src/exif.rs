//! Helpers to extract necessary EXIF information from source images

use crate::Error;
use exif::{Exif, In, Tag, Value};

/// Extract the exif information from the bytes of an image file
///
/// # Errors
/// - failed to extract exif data
pub fn get_exif_data(data: &[u8]) -> Result<Exif, Error> {
    let mut buf_reader = std::io::Cursor::new(data);
    let exif_reader = exif::Reader::new();
    let exif = exif_reader.read_from_container(&mut buf_reader)?;

    Ok(exif)
}

/// Extract the exposure time in seconds from exif information
///
/// # Errors
/// - failed to exposure from exif data
pub fn get_exposures(exif: &Exif) -> Result<f32, Error> {
    match exif
        .get_field(Tag::ExposureTime, In::PRIMARY)
        .ok_or(Error::ExifError(exif::Error::NotFound(
            "ExposureTime not found",
        )))?
        .value
    {
        Value::Rational(ref v) if !v.is_empty() => Ok(v[0].to_f32()),
        _ => Ok(0.),
    }
}

/// Extract the gains from exif information
///
/// # Errors
/// - failed to gains from exif data
#[allow(clippy::cast_precision_loss)]
pub fn get_gains(exif: &Exif) -> Result<f32, Error> {
    match exif
        .get_field(Tag::ISOSpeed, In::PRIMARY)
        .unwrap_or(
            exif.get_field(Tag::StandardOutputSensitivity, In::PRIMARY)
                .unwrap_or(
                    exif.get_field(Tag::PhotographicSensitivity, In::PRIMARY)
                        .ok_or(Error::ExifError(exif::Error::NotFound("ISO not found")))?,
                ),
        )
        .value
    {
        Value::Long(ref v) if !v.is_empty() => Ok(v[0] as f32),
        Value::Short(ref v) if !v.is_empty() => Ok(f32::from(v[0])),
        _ => Ok(0.),
    }
}
