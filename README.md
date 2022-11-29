# image-hdr

This is a rust library which implements the HDR merging algorithm for camera images taken with different exposure times (or with bracketing). It uses the algorithms described in https://www.cl.cam.ac.uk/research/rainbow/projects/noise-aware-merging/2020-ppne-mle.pdf, and uses "Poisson Photon Noise Estimator" equations to estimate final radiances at each pixel position.

## Current State

The library is still in early stages of development, but aims to provide a crate that can handle all HDR merging needs. Towards that end, the following todos are the top priority:

-   Proper error handling – instead of panics, return errors that can be used by users.
-   Ability to provide exposure times and gains manually when metadata is unavailable.
-   Tone mapping algorithm implementations.
-   Improve performance.

## Dependencies

-   image-rs: Uses DynamicImage as the output format and storage format between calculations.
-   rawloader: For supporting RAW image formats.
-   rayon: For doing point calculations in parallel.
-   kamadak-exif: For getting image's metadata, specifically eposure time and gain (ISO).

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

## Contributing

Bug reports and pull requests welcome at https://github.com/anshap1719/image-hdr
