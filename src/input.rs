//! Input type for processing HDR merge

use crate::exif::{get_exif_data, get_exposures, get_gains};
use crate::io::read_image;
use crate::Error;
use image::DynamicImage;
use rayon::prelude::*;
use std::path::Path;
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
    /// returns: `Result<HDRInput, Error>`
    ///
    /// # Errors
    ///
    /// - If image cannot be opened
    /// - If image doesn't contain EXIF metadata for exposure and/or gain.
    pub fn new(path: &Path) -> Result<Self, crate::Error> {
        let new_input = Self::try_from(path)?;

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
    /// returns: `Result<HDRInput, Error>`
    ///
    /// # Errors
    ///
    /// - If image cannot be opened
    /// - invalid gain
    /// - invalid exposure duration
    pub fn with_exposure_and_gain(
        path: &Path,
        exposure: Duration,
        gain: f32,
    ) -> Result<Self, Error> {
        let data = std::fs::read(path)?;
        let image = read_image(&data)?;

        Self::with_image(image, exposure, gain)
    }

    ///
    /// # Arguments
    ///
    /// * `image`:
    /// * `exposure`:
    /// * `gain`:
    ///
    /// returns: `Result<HDRInput, Error>`
    ///
    /// # Errors
    ///
    /// - invalid gain
    /// - invalid exposure duration
    pub fn with_image(image: DynamicImage, exposure: Duration, gain: f32) -> Result<Self, Error> {
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
}

impl TryFrom<&Path> for HDRInput {
    type Error = Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let data = std::fs::read(value)?;
        let image = read_image(&data)?;
        let exif = get_exif_data(&data)?;
        let exposure = get_exposures(&exif)?;
        let gain = get_gains(&exif)?;

        Self::with_image(image, Duration::from_secs_f32(exposure), gain)
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

impl From<Vec<HDRInput>> for HDRInputList {
    fn from(value: Vec<HDRInput>) -> Self {
        Self(value)
    }
}

impl<P: AsRef<Path> + Sync> TryFrom<&[P]> for HDRInputList {
    type Error = Error;

    fn try_from(value: &[P]) -> Result<Self, Self::Error> {
        Ok(HDRInputList(
            value
                .par_iter()
                .map(|value| -> Result<HDRInput, Self::Error> {
                    HDRInput::try_from(value.as_ref())
                })
                .collect::<Result<Vec<HDRInput>, Self::Error>>()?,
        ))
    }
}
