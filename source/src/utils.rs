use ndarray::{Array, Array1, Array2};
use rand::{self, distributions::IndependentSample, Rng};

#[inline(never)]
pub fn init_rnd_array2(
    rng: &mut rand::OsRng,
    shape: &[usize; 2],
    mean: f64,
    std: f64,
) -> Array2<f64> {
    let normal = rand::distributions::Normal::new(mean, std);
    let mut a = Array::zeros((shape[0], shape[1]));
    for e in a.iter_mut() {
        *e = normal.ind_sample(rng);
    }
    a
}

#[inline(never)]
pub fn init_rnd_array1(rng: &mut rand::OsRng, size: usize, mean: f64, std: f64) -> Array1<f64> {
    let normal = rand::distributions::Normal::new(mean, std);
    let mut a = Array1::zeros(size);
    for i in 0..size {
        a[i] = normal.ind_sample(rng);
    }
    a
}

/// Generate `n` challenges with `num_bits`
#[inline(never)]
pub fn get_challenges(n: usize, num_bits: usize) -> Array2<f64> {
    let mut rng = rand::os::OsRng::new().unwrap();
    let mut a = Array::zeros((n, num_bits));
    for m in 0..n {
        for n in 0..num_bits {
            a[[m, n]] = *rng.choose(&[-1.0, 1.0]).unwrap();
        }
    }
    a
}

#[inline(never)]
pub fn sigma_noise(n: u64, sigma_weight: f64, noisiness: f64) -> f64 {
    (n as f64).sqrt() * sigma_weight * noisiness
}

#[inline(never)]
pub fn sign(responses: &mut Vec<f64>) {
    for e in responses {
        if *e < 0.0 {
            *e = -1.0;
        } else if *e > 0.0 {
            *e = 1.0;
        } else {
            *e = 0.0;
        }
    }
}
