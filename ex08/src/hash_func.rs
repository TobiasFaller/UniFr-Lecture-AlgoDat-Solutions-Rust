extern crate rand;

use self::rand::{thread_rng, Rng};

use std::vec::Vec;

pub struct HashMap<T> where T: Clone {
	a: usize,
	b: usize,
	p: usize,
	m: usize,
	buckets: Vec<Vec<(usize, T)>>
}

impl<T> HashMap<T> where T: Clone {
	
	pub fn new(p: usize, m: usize) -> HashMap<T> {
		let mut h = HashMap {
			a: 1,
			b: 1,
			p: p,
			m: m,
			buckets: Vec::with_capacity(m)
		};
		
		h.set_random_parameters();
		
		for _ in 0..m {
			h.buckets.push(Vec::new());
		}
		
		return h;
	}
	
	fn set_random_parameters(&mut self) {
		let mut rng = thread_rng();
		self.a = rng.gen_range(1, self.p);
		self.b = rng.gen_range(0, self.p);
	}
	
	fn apply(&self, value: &usize) -> usize {
		((self.a * value + self.b) % self.p) % self.m
	}
	
	pub fn put(&mut self, key: usize, value: T) {
		let bucket_index = self.apply(&key);
		let mut bucket = &mut self.buckets[bucket_index];
		
		for entry in bucket.iter_mut() {
			if entry.0 == key { // Entry already exists
				entry.1 = value; // Replace the value
				return;
			}
		}
		
		// Entry does not exist -> Append to the bucket
		bucket.push((key, value));
	}
}