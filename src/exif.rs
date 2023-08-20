//! Helpers to extract necessary EXIF information from source images

use crate::Error;
use exif::{Exif, In, Tag, Value};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub(crate) fn get_exif_data(paths: &[String]) -> Result<Vec<Exif>, Error> {
    paths
        .par_iter()
        .map(|path| -> Result<Exif, Error> {
            let file = std::fs::File::open(path)?;
            let mut buf_reader = std::io::BufReader::new(&file);
            let exif_reader = exif::Reader::new();
            let exif = exif_reader.read_from_container(&mut buf_reader)?;

            Ok(exif)
        })
        .collect()
}

pub(crate) fn get_exposures(exif: &[Exif]) -> Result<Vec<f32>, Error> {
    exif.iter()
        .map(|exif| -> Result<f32, Error> {
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
        })
        .collect()
}

#[allow(clippy::cast_precision_loss)]
pub(crate) fn get_gains(exif: &[Exif]) -> Result<Vec<f32>, Error> {
    exif.iter()
        .map(|exif| -> Result<f32, Error> {
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
        })
        .collect()
}
