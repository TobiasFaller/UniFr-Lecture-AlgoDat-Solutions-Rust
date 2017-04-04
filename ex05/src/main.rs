mod hash_func;
mod min_max_avg;

use hash_func::HashFunction;
use hash_func::estimate_c_for_multiple_sets;

const HASH_FUNCTIONS: usize = 10;
const UNIVERSE_SIZE: usize = 100;

const KEY_SETS: usize = 10;
const KEY_SET_SIZE: usize = 20;

fn main() {
	let mut h1 = HashFunction::new(101, 10);
	let (min, max, avg) = estimate_c_for_multiple_sets(&mut h1, KEY_SETS, HASH_FUNCTIONS,
		KEY_SET_SIZE, UNIVERSE_SIZE).unwrap();
	print!("h1: {} {} {}", min, max, avg);
	
	let mut h2 = HashFunction::new(10, 10);
	let (min, max, avg) = estimate_c_for_multiple_sets(&mut h2, KEY_SETS, HASH_FUNCTIONS,
		KEY_SET_SIZE, UNIVERSE_SIZE).unwrap();
	print!("h2: {} {} {}", min, max, avg);
	
}