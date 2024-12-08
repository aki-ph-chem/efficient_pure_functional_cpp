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

pub mod pipe_like_chain {
    pub struct Fp64(pub f64);
    impl Fp64 {
        pub fn sqrt(self) -> Self {
            Self(self.0.sqrt())
        }
    }

    impl std::fmt::Display for Fp64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug, Clone)]
    pub struct VecFp64(pub Vec<f64>);
    impl VecFp64 {
        pub fn mean(self) -> Fp64 {
            Fp64(self.0.iter().sum::<f64>() / self.0.len() as f64)
        }

        pub fn squared(mut self) -> Self {
            for x in self.0.iter_mut() {
                *x *= *x;
            }

            self
        }

        pub fn rms(self) -> Fp64 {
            self.squared().mean().sqrt()
        }
    }
}
