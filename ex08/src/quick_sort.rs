use std::vec::Vec;

pub fn quick_sort<T>(lst: &mut Vec<T>) where T: PartialOrd + Clone {
	let len = &lst.len();
	if len <= &1_usize {
		return;
	}
	
	quick_sort_recursive(lst, 0, (len - 1) as i64);
}

fn quick_sort_recursive<T>(lst: &mut Vec<T>, start: i64, end: i64) where T: PartialOrd + Clone {
	if end - start <= 1{
		return;
	}
	
	let mut i: i64 = start as i64;
	let mut k: i64 = end as i64;
	let pivot = lst[0].clone();
	
	// Sort smaller and bigger elements
	while i < k {
		while i < k && i < end && lst[i as usize] <= pivot {
			i += 1;
		}
		while i <= k && k > start && lst[k as usize] > pivot {
			k -= 1;
		}
		
		if i < k {
			// Swap elements
			lst.swap(k as usize, i as usize);
		}
	}
	
	// Swap pivot into the middle
	lst.swap(start as usize, k as usize);
	
	if k > 0 {
		quick_sort_recursive(lst, start, k - 1);
	}
	quick_sort_recursive(lst, k + 1, end);
}