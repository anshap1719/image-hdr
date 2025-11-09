# image-hdr ![](https://github.com/anshap1719/image-hdr/actions/workflows/rust.yml/badge.svg)

This is a rust library which implements the HDR merging algorithm for camera images taken with different exposure
times (or with bracketing). It uses the algorithms described
in https://www.cl.cam.ac.uk/research/rainbow/projects/noise-aware-merging/2020-ppne-mle.pdf, and uses "Poisson Photon
Noise Estimator" equations to estimate final radiances at each pixel position.

## Current State

The library is still in early stages of development, but aims to provide a crate that can handle all HDR merging needs.
Towards that end, the following todos are the top priority:

- Tone mapping algorithm implementations.
- Improve performance.

## Dependencies

- image-rs: Uses DynamicImage as the output format and storage format between calculations.
- rawloader: For supporting RAW image formats.
- rayon: For doing point calculations in parallel.
- kamadak-exif: For getting image's metadata, specifically exposure time and gain (ISO).

## Usage

```
let paths = vec!["src/image1.tif", "src/image2.tif", "src/image3.tif"];
let hdr_merge = image_hdr::hdr_merge_images(paths);
let stretched = apply_histogram_stretch(&fusion);

stretched
    .to_rgba16()
    .save(format!("src/hdr_merged.tiff"))
    .unwrap();
```

## Samples

### Given the following 3 exposures:

![alt "1/640s"](https://github.com/user-attachments/assets/476fc627-fcf3-480c-9dc4-b9c868bf8462)
![alt "1/4000s"](https://github.com/user-attachments/assets/3ed11fd1-2ab0-494c-8c7f-d5216e40cd4e)
![alt "1/80s"](https://github.com/user-attachments/assets/bbb01cd8-91c3-4431-854b-318eeaa07ad0)

### Resulting unprocessed image:

![alt "Merged image"](https://github.com/user-attachments/assets/f19b5234-e2e6-447d-a05e-25389ca38d39)

### After basic processing (Levels and Contrast):

![alt "Processed image"](https://github.com/user-attachments/assets/f2214838-8990-497f-8448-a2da1d368211)

## Contributing

Bug reports and pull requests welcome at https://github.com/anshap1719/image-hdr

## Citations

- Noise-Aware Merging of High Dynamic Range Image Stacks without Camera Calibration by Param Hanji, Fangcheng Zhong, and
  Rafa l K. Mantiuk (https://www.cl.cam.ac.uk/~rkm38/pdfs/hanji2020_noise_aware_HDR_merging.pdf)
