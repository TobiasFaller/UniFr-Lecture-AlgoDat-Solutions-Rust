extern crate getopts;
extern crate rand;
extern crate time;

mod hash_func;
mod quick_sort;

use hash_func::HashMap;
use quick_sort::quick_sort;

use getopts::Options;
use rand::{Rng, thread_rng};
use std::env;
use std::f64;

fn print_usage(program: &str, opts: Options) {
    println!("{}", opts.usage(&format!("Usage: {} [options]", program)));
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let program = &args[0];
	
	let mut opts = Options::new();
	opts.optflag("h", "help", "Shows the help for this program.");
	opts.optopt("l", "lin", "Test with linear distances", "SWITCH");
	opts.optopt("e", "exp", "Test with exponential distances", "SWITCH");
	
	let matches = match opts.parse(&args[1..]) {
		Ok(m) => { m },
		Err(e) => { panic!(e.to_string()) }
	};
	
	if matches.opt_present("h") {
		print_usage(&program, opts);
        return;
	}
	
	if matches.opt_present("l") {
		measure_lin();
	}
	if matches.opt_present("e") {
		measure_exp();
	}
	
	if !matches.opt_present("l") && !matches.opt_present("e") {
		measure_lin();
	}
}

fn measure_lin() {
	println!("Measuring with linear steps");
	
	let size: u64 = 2_u64.pow(19); // The maximum size the test might reach
    let samples: u64 = 10; // The number of sample points to measure
    
    let factor = size / samples;
    
    for sample in 0..samples + 1 {
    	let result = measure_runtime(sample * factor);
	    println!("{}\t{:.5}\t{:.5}", result.0, result.1, result.2);
    }
}

fn measure_exp() {
	println!("Measuring with exponential steps");
	
	let min: u64 = 6; // The minimum size with which to start
    let max: u64 = 20; // The maximum size the test might reach
    let samples: u64 = 15; // The number of sample points to measure
    
    let factor: f64 = ((max - min + 1) as f64) / (samples as f64);

    for sample in 0..samples {
    	let result = measure_runtime(2_f64.powf(sample as f64 * factor + min as f64) as u64);
	    println!("{}\t{:.5}\t{:.5}", result.0, result.1, result.2);
    }
}

fn measure_runtime(n: u64) -> (usize, f64, f64) {
	let mut map_time: u64 = 0;
	let mut sort_time: u64 = 0;
	
	for _ in 0..3 {
		let lst_data = gen_rand_lst(n as usize);
		
		let map_lst = lst_data.clone(); 
		let mut sort_lst = lst_data;
		
		let start_time = time::get_time();
		let mut h: HashMap<u32> = HashMap::new(2_usize.pow(31) - 1, 2_usize.pow(15));
		for i in map_lst {
			h.put(i, 0);
		}
		map_time += (time::get_time() - start_time).num_milliseconds() as u64;
		
		let start_time = time::get_time();
		quick_sort(&mut sort_lst);
		sort_time += (time::get_time() - start_time).num_milliseconds() as u64;
	}
	
	return (n as usize, map_time as f64 / 3000.0, sort_time as f64 / 3000.0);
}

fn gen_rand_lst(size: usize) -> Vec<usize> {
	let mut values = Vec::with_capacity(size);
	
	for i in 0..size {
		values.push(i);
	}
	
	let mut rng = thread_rng();
	rng.shuffle(&mut values);
	
	return values;
}