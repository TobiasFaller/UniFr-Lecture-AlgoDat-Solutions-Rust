use std::clone::Clone;
use std::cmp::Ord;
use std::vec::Vec;

pub fn insertion_sort<T>(input: Vec<T>) -> Vec<T> where T: Ord + Clone {
	fn insert<U>(mut acc: Vec<U>, x: &U) -> Vec<U> where U: Ord + Clone {
		let mut pos: usize = 0;
		
		for val in acc.iter() {
			if val > &x {
				break;
			}
			
			pos += 1;
		}
		
		acc.insert(pos, x.clone());
		acc
	}
	
	input.iter().fold(Vec::<T>::new(), |acc, x| insert(acc, x))
}