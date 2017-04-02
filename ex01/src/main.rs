mod ins_sort;

use std::vec::Vec;
use ins_sort::ins_sort::insertion_sort;

fn main() {
	let data: Vec<i32> = vec![5, 3, 1, 54, 23, 1];
	let sorted: Vec<i32> = insertion_sort(data.clone());
	println!("{:?}", data);
	println!("{:?}", sorted);
}