use exif::{Tag, In, Value};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub(crate) fn get_exposures(paths: &[String]) -> Vec<f32> {
    paths.par_iter().map(|path| {
        let file = std::fs::File::open(path).unwrap();
        let mut buf_reader = std::io::BufReader::new(&file);
        let exif_reader = exif::Reader::new();
        let exif = exif_reader.read_from_container(&mut buf_reader).unwrap();

        let exposure = match exif
            .get_field(Tag::ExposureTime, In::PRIMARY)
            .unwrap()
            .value
        {
            Value::Rational(ref v) if !v.is_empty() => v[0].to_f32(),
            _ => 0.,
        };

        exposure
    }).collect()
}

pub(crate) fn get_gains(paths: &[String]) -> Vec<f32> {
    paths.par_iter().map(|path| {
        let file = std::fs::File::open(path).unwrap();
        let mut buf_reader = std::io::BufReader::new(&file);
        let exif_reader = exif::Reader::new();
        let exif = exif_reader.read_from_container(&mut buf_reader).unwrap();

        let gain = match exif
            .get_field(Tag::ISOSpeed, In::PRIMARY)
            .unwrap_or(
                exif.get_field(Tag::StandardOutputSensitivity, In::PRIMARY).unwrap_or(
                    exif.get_field(Tag::PhotographicSensitivity, In::PRIMARY).unwrap()
                )
            )
            .value
        {
            Value::Long(ref v) if !v.is_empty() => v[0] as f32,
            Value::Short(ref v) if !v.is_empty() => v[0] as f32,
            _ => 0.,
        };

        gain
    }).collect()
}
