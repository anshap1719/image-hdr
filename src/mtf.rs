use ndarray::{Array1, Array2, Axis};
use ndarray_stats::QuantileExt;

struct Stretch {
    shadows_clip: f64,
    target_bkg: f64,
}

impl Stretch {
    fn new(target_bkg: f64, shadows_clip: f64) -> Self {
        Stretch {
            shadows_clip,
            target_bkg,
        }
    }

    fn get_avg_dev(&self, data: &Array2<f64>) -> f64 {
        let median = data.quantile(0.5).unwrap();
        let n = data.len() as f64;
        let median_deviation = data.mapv(|x| (x - median).abs());
        let avg_dev = median_deviation.sum() / n;
        avg_dev
    }

    fn mtf(&self, m: f64, x: &Array1<f64>) -> Array1<f64> {
        let mut result = x.clone();
        for i in 0..result.len() {
            if result[i] == 0.0 {
                result[i] = 0.0;
            } else if result[i] == m {
                result[i] = 0.5;
            } else if result[i] == 1.0 {
                result[i] = 1.0;
            } else {
                result[i] = (m - 1.0) * result[i] / (((2.0 * m - 1.0) * result[i]) - m);
            }
        }
        result
    }

    fn get_stretch_parameters(&self, data: &Array2<f64>) -> (f64, f64, f64) {
        let median = data.quantile(0.5).unwrap();
        let avg_dev = self.get_avg_dev(data);

        let c0 = median + (self.shadows_clip * avg_dev).max(0.0).min(1.0);
        let m = self
            .mtf(self.target_bkg, &Array1::from(vec![median - c0]))
            .into_iter()
            .next()
            .unwrap();

        (c0, 1.0, m)
    }

    fn stretch(&self, data: &Array2<f64>) -> Array2<f64> {
        let d = data / data.fold(0.0, |max, &x| max.max(x));
        let (c0, c1, m) = self.get_stretch_parameters(&d);

        let mut result = d.clone();
        for i in 0..result.len() {
            if result[i] < c0 {
                result[i] = 0.0;
            } else {
                result[i] = self
                    .mtf(m, &Array1::from(vec![(result[i] - c0) / (1.0 - c0)]))
                    .into_iter()
                    .next()
                    .unwrap();
            }
        }
        result
    }
}

fn apply_stretch(data: Array2<f64>, target_bkg: f64, shadows_clip: f64) -> Array2<f64> {
    Stretch::new(target_bkg, shadows_clip).stretch(&data)
}
