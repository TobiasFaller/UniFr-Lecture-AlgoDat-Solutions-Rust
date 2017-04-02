mod heap_sort;

use std::vec::Vec;
use heap_sort::heap_sort;

fn main() {
	let mut data: Vec<i32> = vec![5, 3, 1, 54, 23, 1];
	println!("{:?}", data);
	
	heap_sort(&mut data);
	println!("{:?}", data);
}