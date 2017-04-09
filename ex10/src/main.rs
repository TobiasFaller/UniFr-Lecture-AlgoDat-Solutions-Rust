#![feature(box_syntax, box_patterns)]

extern crate rand; 
extern crate time;

mod binary_search_tree;

use binary_search_tree::BinarySearchTree;

use rand::{Rng, thread_rng};
use time::get_time;

fn main() {
	let size: u64 = 2_u64.pow(12); // The maximum size the test might reach
    let samples: u64 = 11; // The number of sample points to measure
    
    println!("n\trand\tdepth\tlin\tdepth");
    for sample in 0..samples + 1 {
    	let result = measure_runtime(sample * size / samples);
	    println!("{}\t{:.5}\t{}\t{:.5}\t{}", result.0, result.1, result.2, result.3, result.4);
    }
}

fn measure_runtime(n: u64) -> (u64, u64, usize, u64, usize) {
	let rand_data = box gen_rand_lst(n as usize);
	let lin_data = box gen_lin_lst(n as usize);
	
	let start_time = get_time();
	let mut t1: Box<BinarySearchTree<usize, String>> = box BinarySearchTree::new();
	for i in rand_data.iter() {
		t1.insert(*i, String::default());
	}
	let rand_time = (get_time() - start_time).num_milliseconds() as u64;
	
	let start_time = get_time();
	let mut t2: Box<BinarySearchTree<usize, String>> = box BinarySearchTree::new();
	for i in lin_data.iter() {
		t2.insert(*i, String::default());
	}
	let lin_time = (get_time() - start_time).num_milliseconds() as u64;
	
	return (n, rand_time, t1.depth(), lin_time, t2.depth());
}

fn gen_rand_lst(size: usize) -> Box<Vec<usize>> {
	let mut values = gen_lin_lst(size);
	
	let mut rng = thread_rng();
	rng.shuffle(values.as_mut());
	
	return values;
}

fn gen_lin_lst(size: usize) -> Box<Vec<usize>> {
	let mut values = box Vec::with_capacity(size);
	
	for i in 0..size {
		values.push(i);
	}
	
	return values;
}