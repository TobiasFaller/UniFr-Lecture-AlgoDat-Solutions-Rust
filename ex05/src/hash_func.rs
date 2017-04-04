extern crate rand;

use rand;

use std::collections::HashSet;
use std::f64;

struct HashFunction {
	a: usize,
	b: usize,
	p: usize,
	m: usize
}

impl HashFunction {
	
	const rng: &'static rand::Rng = rand::Rng::new();
	
	fn new(p: usize, m: usize) -> HashFunction {
		let mut h = HashFunction {
			a: 1,
			b: 1,
			p: p,
			m: m
		};
		
		h.set_random_parameters();
		
		return h;
	}
	
	fn set_random_parameters(&mut self) {
		a = rng.gen::<usize>() % p;
		b = rng.gen::<usize>() % p;
	}
	
	fn apply(&self, value: usize) -> usize {
		((a * value + b) % p) % m
	}
	
	fn num_buckets(&self) -> usize {
		m
	}
}

fn mean_bucket_size(h: &HashFunction, s: &Vec<usize>) -> f64 {
	let mut buckets = HashSet::new();
	
	for key in s {
		buckets.insert(h.apply(key));
	}
	
	return (s.len() as f64) / (buckets.len() as f64);
}

fn estimate_c_for_single_set(h: &HashFunction, s: &Vec<usize>) -> f64 {
	let mut best_c = f64::MAX;
	
	for i in 0..1000 {
		h.set_random_parameters();
		
		let c = (mean_bucket_size(h, s) - 1.0) * (h.num_buckets() as f64) / (s.len() as f64);
		best_c = c.max(best_c);
	}
	
	return best_c;
}