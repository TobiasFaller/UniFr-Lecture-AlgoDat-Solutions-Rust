extern crate rand;

use min_max_avg::MinMaxAvg;

use self::rand::{sample, thread_rng, Rng};

use std::f64;
use std::vec::Vec;
use std::collections::HashSet;

pub struct HashFunction {
	a: usize,
	b: usize,
	p: usize,
	m: usize
}

impl HashFunction {
	
	pub fn new(p: usize, m: usize) -> HashFunction {
		let mut h = HashFunction {
			a: 1,
			b: 1,
			p: p,
			m: m
		};
		
		h.set_random_parameters();
		
		return h;
	}
	
	pub fn set_random_parameters(&mut self) {
		let mut rng = thread_rng();
		self.a = rng.gen_range(1, self.p);
		self.b = rng.gen_range(0, self.p);
	}
	
	pub fn apply(&self, value: &usize) -> usize {
		((self.a * value + self.b) % self.p) % self.m
	}
	
	pub fn num_buckets(&self) -> usize {
		self.m
	}
}

pub fn mean_bucket_size(h: &HashFunction, s: &Vec<usize>) -> f64 {
	let mut buckets = HashSet::new();
	
	for key in s {
		buckets.insert(h.apply(key));
	}
	
	return (s.len() as f64) / (buckets.len() as f64);
}

pub fn estimate_c_for_single_set(h: &mut HashFunction, s: &Vec<usize>, h_funcs: usize) -> f64 {
	let mut best_c = f64::MAX;
	
	for _ in 0..h_funcs {
		h.set_random_parameters();
		
		let c = (mean_bucket_size(h, s) - 1.0_f64) * (h.num_buckets() as f64) / (s.len() as f64);
		best_c = c.max(best_c);
	}
	
	return best_c;
}

pub fn estimate_c_for_multiple_sets(h: &mut HashFunction, n: usize, h_funcs: usize, k: usize, u: usize) -> Option<(f64, f64, f64)> {
	let mut c: Vec<f64> = Vec::new();
	c.reserve_exact(n);
	
	for _ in 0..n {
		let key_set = generate_key_set(k, u);
		c.push(estimate_c_for_single_set(h, &key_set, h_funcs));
	}
	
	return c.iter().collect::<MinMaxAvg<f64>>().get();
}

fn generate_key_set(k: usize, u: usize) -> Vec<usize> {
	let mut rng = thread_rng();
	return sample(&mut rng, 0..u, k);
}