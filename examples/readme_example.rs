//! The example from the Readme.md
//!
//! Run with `cargo run --example readme_example --no-default-features`

use std::{io::Read, time::Duration};

use image_hdr::{
    exif::{get_exif_data, get_exposures, get_gains},
    input::HDRInput,
    stretch::apply_histogram_stretch,
};

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
enum Error {
    Reqwest(#[from] reqwest::Error),
    Io(#[from] std::io::Error),
    Image(#[from] image::ImageError),
    ImageHDR(#[from] image_hdr::Error),
}

fn main() -> Result<(), Error> {
    let image_urls = [
        (
            "https://image-hdr-assets.s3.ap-south-1.amazonaws.com/DSC00001+Large.jpeg",
            1.0 / 640.0,
        ),
        (
            "https://image-hdr-assets.s3.ap-south-1.amazonaws.com/DSC00002+Large.jpeg",
            1.0 / 4000.0,
        ),
        (
            "https://image-hdr-assets.s3.ap-south-1.amazonaws.com/DSC00003+Large.jpeg",
            1.0 / 80.0,
        ),
    ];

    let mut images = Vec::with_capacity(image_urls.len());
    let mut buf = Vec::new();
    for (url, exposure) in image_urls {
        let file_name = url
            .split('/')
            .next_back()
            .expect("Expected filename as last component in url");

        if std::path::Path::exists(file_name.as_ref()) {
            println!("Using cached image: {url}");

            let _ = std::fs::File::open(file_name)?.read_to_end(&mut buf)?;
        } else {
            let mut response = reqwest::blocking::get(url)?;
            println!("Downloading image: {url}");

            let _ = response.read_to_end(&mut buf)?;
            // we ignore failing to cache the image
            let _ = std::fs::write(file_name, &buf);
        }

        let exif = get_exif_data(&buf)?;
        let gains = get_gains(&exif).unwrap_or(1.0);
        let exposure = get_exposures(&exif).unwrap_or(exposure);

        println!("Loading image: {url}");

        let image = image::load_from_memory_with_format(&buf, image::ImageFormat::Jpeg)?;

        println!("Adding image: {url}");
        dbg!(gains, exposure);

        images.push(HDRInput::with_image(
            &image,
            Duration::from_secs_f32(exposure),
            gains,
        )?);

        buf.clear();
    }

    println!("Mergin images...");
    let hdr_merged = image_hdr::hdr_merge_images(&mut images.into())?;
    let stretched = apply_histogram_stretch(&hdr_merged)?;

    println!("Saving merged image...");
    stretched
        .to_rgba16()
        .save("./DSC00001-DSC00003+Large.tiff")?;

    Ok(())
}
