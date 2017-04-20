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

#[test]
fn test_normal_sort() {
	let data = vec![5, 10, 3, 1, 2, 564, 874, 21, 454, 12, 5];
	assert_eq!(vec![1, 2, 3, 5, 5, 10, 12, 21, 454, 564, 874], insertion_sort(data));
}

#[test]
fn test_empty() {
	assert_eq!(vec![] as Vec<usize>, insertion_sort(vec![] as Vec<usize>));
}