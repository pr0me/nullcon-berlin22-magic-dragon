use crate::utils;
use ndarray::{self, concatenate, s, Array1, Array2, Array3, Axis};
use ndarray_einsum_beta::{self, einsum};
use rand;
use std::io::Error;

pub struct XORArbiterPUF {
    weight_array: Array2<f64>,
    bias: Array1<f64>,
    noise: f64,
}

impl XORArbiterPUF {
    #[inline(never)]
    pub fn new(n: usize, k: usize, noisiness: f64, biased: bool) -> Result<Self, Error> {
        let noise = utils::sigma_noise(n as _, 1.0, noisiness);

        let mut rng = rand::os::OsRng::new()?;
        let weights_0 = utils::init_rnd_array2(&mut rng, &[k, 1], 0.0, 0.5);
        let weights_1 = utils::init_rnd_array2(&mut rng, &[k, n - 1], 0.0, 1.0);
        let weight_array = concatenate![Axis(1), weights_0, weights_1];

        let bias: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>>;
        if biased {
            bias = utils::init_rnd_array1(&mut rng, k, 0.0, 0.5);
        } else {
            bias = Array1::zeros(k);
        }

        Ok(Self {
            weight_array,
            bias,
            noise,
        })
    }

    #[inline(never)]
    fn arbiter_threshold_transform(&self, sub_challenges: &mut Array3<f64>) {
        let n = sub_challenges.shape()[2];
        let mut i = n - 2;
        while i != usize::MAX {
            for k in 0..(sub_challenges.shape()[0]) {
                for j in 0..(sub_challenges.shape()[1]) {
                    sub_challenges[[k, j, i]] *= sub_challenges[[k, j, i + 1]];
                }
            }
            i = i.wrapping_sub(1);
        }
    }

    #[inline(never)]
    fn transform_id(&self, challenges: &Array2<f64>, k: u64) -> Array3<f64> {
        let shape = challenges.shape();
        challenges
            .broadcast((k as _, shape[0] as _, shape[1] as _))
            .unwrap()
            .permuted_axes([1, 0, 2])
            .to_owned()
    }

    #[inline(never)]
    fn transform_atf(&self, challenges: &Array2<f64>, k: i64) -> Array3<f64> {
        let shape = challenges.shape();
        let mut sub_challenges = self.transform_id(&challenges, 1);

        self.arbiter_threshold_transform(&mut sub_challenges);
        sub_challenges
            .permuted_axes([1, 0, 2])
            .broadcast((k as _, shape[0] as _, shape[1] as _))
            .unwrap()
            .permuted_axes([1, 0, 2])
            .to_owned()
    }

    #[inline(never)]
    fn combiner_xor(&self, responses: Array2<f64>) -> Vec<f64> {
        let mut vals: Vec<f64> = Vec::new();
        for a in 0..(responses.shape()[0] as usize) {
            vals.push(responses.slice(s![a, ..]).product());
        }
        vals
    }

    #[inline(never)]
    fn ltf_eval(&self, sub_challenges: &Array3<f64>) -> Array2<f64> {
        let _k = self.weight_array.shape()[0];
        let _n = self.weight_array.shape()[1] - 1;
        let unbiased = einsum("ji,kji->kj", &[&self.weight_array, sub_challenges])
            .unwrap()
            .into_dimensionality::<ndarray::Ix2>()
            .unwrap();
        let evaled = unbiased + &self.bias;
        let mut rng = rand::os::OsRng::new().unwrap();
        let noise = utils::init_rnd_array2(&mut rng, &[evaled.shape()[0], 4], 0.0, self.noise);
        return evaled + noise;
    }

    #[inline(never)]
    pub fn eval(&self, challenges: &Array2<f64>) -> Vec<f64> {
        let n = self.weight_array.shape()[0];
        let mut responses =
            self.combiner_xor(self.ltf_eval(&self.transform_atf(challenges, n as _)));
        utils::sign(&mut responses);
        responses
    }
}
