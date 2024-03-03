//! Input type for processing HDR merge

use crate::exif::{get_exif_data, get_exposures, get_gains};
use crate::io::read_image;
use crate::Error;
use image::DynamicImage;
use rayon::prelude::*;
use std::time::Duration;

/// Base input item that is used to process the HDR merge
#[derive(Clone)]
pub struct HDRInput {
    image: DynamicImage,
    exposure: f32,
    gain: f32,
}

impl HDRInput {
    /// Create new [`HDRInput`] from a given file path. The file must have EXIF data for exposure
    /// and gain.
    ///
    /// # Arguments
    ///
    /// * `path`: Path to file
    ///
    /// returns: Result<HDRInput, Error>
    ///
    /// # Errors
    ///
    /// - If image cannot be opened
    /// - If image doesn't contain EXIF metadata for exposure and/or gain.
    pub fn new(path: &String) -> Result<Self, crate::Error> {
        let new_input = Self::try_from(path.to_string())?;

        Ok(new_input)
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `path`:
    /// * `exposure`:
    /// * `gain`:
    ///
    /// returns: Result<HDRInput, Error>
    ///
    /// # Errors
    ///
    /// - If image cannot be opened
    pub fn with_exposure_and_gain(
        path: &String,
        exposure: Duration,
        gain: f32,
    ) -> Result<Self, Error> {
        let image = read_image(path)?;

        if gain.is_infinite() || gain.is_nan() || gain <= 0. {
            return Err(Error::InputError {
                parameter_name: "gain".to_string(),
                message: "Gain must be a valid positive and non-zero floating point number"
                    .to_string(),
            });
        }

        if exposure.is_zero() {
            return Err(Error::InputError {
                parameter_name: "exposure".to_string(),
                message: "Exposure must be a positive non-zero duration".to_string(),
            });
        }

        Ok(Self {
            image,
            exposure: exposure.as_secs_f32(),
            gain,
        })
    }

    /// Get exposure of the input item
    #[must_use]
    pub fn get_exposure(&self) -> f32 {
        self.exposure
    }

    /// Get gain of the input item
    #[must_use]
    pub fn get_gain(&self) -> f32 {
        self.gain
    }

    /// Get underlying image data for the input item
    #[must_use]
    pub fn get_image(&self) -> &DynamicImage {
        &self.image
    }

    /// Get underlying image data for the input item as mutable
    #[must_use]
    pub fn get_image_mut(&mut self) -> &mut DynamicImage {
        &mut self.image
    }
}

impl TryFrom<String> for HDRInput {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let image = read_image(&value)?;
        let exif = get_exif_data(&value)?;
        let exposure = get_exposures(&exif)?;
        let gain = get_gains(&exif)?;

        Ok(Self {
            image,
            exposure,
            gain,
        })
    }
}

/// A wrapper for list of [`HDRInput`] for ease of trait implementations.
pub struct HDRInputList(Vec<HDRInput>);

impl HDRInputList {
    /// Get list of [`HDRInput`] as a vec.
    #[must_use]
    pub fn into_vec(self) -> Vec<HDRInput> {
        self.0
    }

    /// Get list of [`HDRInput`] as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[HDRInput] {
        &self.0
    }

    /// Get list of [`HDRInput`] as a mutable slice.
    #[must_use]
    pub fn as_slice_mut(&mut self) -> &mut [HDRInput] {
        &mut self.0
    }

    /// Returns the number of elements in the list
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the vector contains no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl TryFrom<&[String]> for HDRInputList {
    type Error = Error;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        Ok(HDRInputList(
            value
                .par_iter()
                .map(|value| -> Result<HDRInput, Self::Error> {
                    let image = read_image(value)?;
                    let exif = get_exif_data(value)?;
                    let exposure = get_exposures(&exif)?;
                    let gain = get_gains(&exif)?;

                    Ok(HDRInput {
                        image,
                        exposure,
                        gain,
                    })
                })
                .collect::<Result<Vec<HDRInput>, Self::Error>>()?,
        ))
    }
}
