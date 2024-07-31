use image_hdr::hdr_merge_images;
use image_hdr::input::HDRInputList;
use image_hdr::stretch::apply_histogram_stretch;

fn main() {
    let paths = vec![
        "DSC00001.jpeg".to_string(),
        "DSC00002.jpeg".to_string(),
        "DSC00003.jpeg".to_string(),
    ];
    let hdr_merge = hdr_merge_images(HDRInputList::try_from(paths.as_slice()).unwrap()).unwrap();
    let stretched = apply_histogram_stretch(&hdr_merge).unwrap();

    stretched
        .to_rgba16()
        .save(format!("hdr_merged.tiff"))
        .unwrap();
}
