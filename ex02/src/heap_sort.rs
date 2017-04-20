use std::cmp::Ord;
use std::vec::Vec;

pub fn heap_sort<T>(lst: &mut Vec<T>) where T: Ord {
	let lst_size: usize = lst.len();
	if lst_size <= 1 {
		return;
	}
	
	// Create the initial heap
	heapify(lst);
	
	for index in (1 .. lst_size).rev() {
		// Swap the max to the end
		lst.swap(0, index);
		
		// Repair the heap
		repair_heap(lst, 0, index);
	}
}

fn heapify<T>(lst: &mut Vec<T>) where T: Ord {
	let lst_size: usize = lst.len();
	
	// Create heap updating the heap condition from the bottom layer
	// We only need to start after the first half (ignore the leaves)
	for index in (0 .. lst_size / 2 - 1).rev() {
		repair_heap(lst, index, lst_size);
	}
}

fn repair_heap<T>(lst: &mut Vec<T>, mut index: usize, size: usize) where T: Ord {
	// We swap the elements downwards to build the heap from the bottom up
	loop {
		let left = index * 2 + 1;
		let right = index * 2 + 2; // left + 1
		let last_index = size - 1;
		
		// If the parent element is larger swap the parent element
		// with the child. We have to explicitly choose the child.
		
		let mut max: usize = index;
		if left <= last_index && lst[max] < lst[left] {
			max = left;
		}
		if right <= last_index && lst[max] < lst[right] {
			max = right;
		}
		
		if max == index {
			// Heap condition is created
			break
		}
		
		lst.swap(max, index);
		
		// Repair downwards
		index = max;
	}
}

#[test]
fn test_normal_sort() {
	let mut data = vec![5, 10, 3, 1, 2, 564, 874, 21, 454, 12, 5];
	heap_sort(&mut data);
	
	assert_eq!(vec![1, 2, 3, 5, 5, 10, 12, 21, 454, 564, 874], data);
}

#[test]
fn test_empty() {
	let mut data = vec![] as Vec<usize>;
	heap_sort(&mut data);
	
	assert_eq!(vec![] as Vec<usize>, data);
}