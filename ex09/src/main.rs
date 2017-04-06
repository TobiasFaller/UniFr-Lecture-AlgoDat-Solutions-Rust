use std::f64;

fn calculate_runtime(n: usize) -> usize {
	if n <= 1 {
		return 1;
	}
	
	return 4 * calculate_runtime(n / 2) + n.pow(2);
}

fn main() {
	// Define the number of sample points and the maximum input size
	let samples = 100;
	let max = 1000000;

	// Define the factor for upper / lower bound
	// We assume n^2 log(n) runtime
	let a = 1.0;
	let b = 2.0;
	
	let factor = max / samples;

	// Print a csv header with 'tab' as separator
	println!("n\tmin\tf\tmax");

	// Calculate and print the runtime for each sample
	for sample in 1_usize..samples + 1 {
		let n: f64 = (sample * factor) as f64;
		println!("{:.0}\t{:.0}\t{:.0}\t{:.0}",
			n,
			a * n.powf(2.0) * f64::log10(n),
			calculate_runtime(n as usize),
			b * n.powf(2.0) * f64::log10(n)
		);
	}
}