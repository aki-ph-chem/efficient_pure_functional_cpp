fn sum(v: &[f64]) -> f64 {
    v.iter().sum()
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

pub mod use_ref_mut {
    use super::*;

    pub fn squared(v: &mut Vec<f64>) {
        for x in v.iter_mut() {
            *x *= *x;
        }
    }

    pub fn rms(v: &mut Vec<f64>) -> f64 {
        squared(v);
        mean(v).sqrt()
    }
}

pub mod use_copy_or_move {
    use super::*;

    pub fn squared(v: Vec<f64>) -> Vec<f64> {
        let mut v = v;
        for x in v.iter_mut() {
            *x *= *x;
        }

        v
    }

    pub fn rms(v: Vec<f64>) -> f64 {
        mean(&squared(v)).sqrt()
    }
}
