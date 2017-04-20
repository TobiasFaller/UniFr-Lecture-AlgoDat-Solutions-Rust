use std::cmp::min;

pub fn compute_ed_recursively(x: &str, y: &str) -> usize {
	let n = x.len();
	let m = y.len();
	
	if n == 0 {
		return m;
	}
	if m == 0 {
		return n;
	}
	
	let ed_a = compute_ed_recursively(x, &y[0..m-1]) + 1;
	let ed_b = compute_ed_recursively(&x[0..n-1], y) + 1;
	let mut ed_diag = compute_ed_recursively(&x[0..n-1], &y[0..m-1]);
	if x.as_bytes()[n-1] != y.as_bytes()[m-1] {
		ed_diag += 1;
	}
	
	return min(ed_a, min(ed_b, ed_diag));
}

pub fn compute_ed_via_table(x: &str, y: &str) -> usize {
	let n = x.len();
	let m = y.len();
	
	if n == 0 {
		return m;
	}
	if m == 0 {
		return n;
	}
	
	let mut data_raw = box vec![0_usize; (n + 1) * (m + 1)];
	let mut data_base: Vec<_> = data_raw.as_mut_slice().chunks_mut(m + 1).collect();
	let mut array: &mut [&mut [_]] = data_base.as_mut_slice();
	
	for i in 0..n + 1 {
		for j in 0..m + 1 {
			if i == 0 {
				array[i][j] = j;
				continue;
			}
			
			if j == 0 {
				array[i][j] = i;
				continue;
			}
			
			let a = array[i - 1][j] + 1;
			let b = array[i][j - 1] + 1;
			let mut diag = array[i - 1][j - 1];
			
			if &x.as_bytes()[i - 1] != &y.as_bytes()[j - 1] {
				diag += 1;
			}
			
			array[i][j] = min(a, min(b, diag));
		}
	}
	
	return array[n][m];
}

#[test]
fn test_distance() {
	assert_eq!(2, compute_ed_recursively("donald", "ronaldo"));
	assert_eq!(2, compute_ed_via_table("donald", "ronaldo"));

	assert_eq!(2, compute_ed_recursively("grau", "raum"));
	assert_eq!(2, compute_ed_via_table("grau", "raum"));

	assert_eq!(4, compute_ed_recursively("Hello", "Hi"));
	assert_eq!(4, compute_ed_via_table("Hello", "Hi"));
}

#[test]
fn test_border_cases() {
	assert_eq!(0, compute_ed_recursively("", ""));
	assert_eq!(0, compute_ed_via_table("", ""));

	assert_eq!(11, compute_ed_recursively("Hello World", ""));
	assert_eq!(11, compute_ed_via_table("Hello World", ""));

	assert_eq!(11, compute_ed_recursively("", "Hello World"));
	assert_eq!(11, compute_ed_via_table("", "Hello World"));
}