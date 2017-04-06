extern crate getopts;
extern crate time;

mod dynamic_array;

use dynamic_array::DynamicArray;

use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    println!("{}", opts.usage(&format!("Usage: {} [options]", program)));
}

fn test_1_or_2(append: bool) {
	match append {
		true => println!("Running test 1 (append)"),
		false => println!("Running test 2 (remove)")
	};
	
	let size: usize = 100000000; // Number of inserts / removes
	let samples: usize = 100; // 100 measure points
	
	let factor = size / samples; // Number of steps per sample
	
	let mut a: DynamicArray<i64> = DynamicArray::new();
	
	if !append { // Initialize array with 10 million elements
		for _ in 0..size {
			a.append(0);
		}
	}
	
	let func: Box<Fn(&mut DynamicArray<_>)>;
	if append {
		func = Box::new(|a| {
			for _ in 0..factor {
				a.append(0);
			}
		});
	} else {
		func = Box::new(|a| {
			for _ in 0..factor {
				a.remove();
			}
		});
	}
	
	let start_time = time::get_time();
	for n in 1..samples + 1 {
		func(&mut a);
		let delta_time = (time::get_time() - start_time).num_milliseconds();
		println!("{}\t{:.5}", n * factor, delta_time);
	}
}

fn test_3_or_4(append: bool) {
	match append {
		true => println!("Running test 3 (append and remove)"),
		false => println!("Running test 4 (remove and append)")
	};

	let size: usize = 100000000; // Number of inserts / removes
	let samples: usize = 100; // 100 measure points
	
	let factor = size / samples; // Number of steps per sample
	
	let mut a: DynamicArray<i64> = DynamicArray::new();
	for _ in 0..size {
		a.append(0);
	}
	
	let start_time = time::get_time();
	
	let mut append = append;
	for n in 1..samples + 1 {
		let mut last_size = a.capacity();
		for _ in 0..factor {
			if append {
				a.append(0);
			} else {
				a.remove();
			}
			
			let new_size = a.capacity();
			append ^= new_size != last_size;
			last_size = new_size;
		}
		
		let delta_time = (time::get_time() - start_time).num_milliseconds();
		println!("{}\t{:.5}", n * factor, delta_time);
	}
}

fn run_all() {
	println!("Running all tests!");
	test_1_or_2(true);
	test_1_or_2(false);
	test_3_or_4(true);
	test_3_or_4(false);
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let program = &args[0];
	
	let mut opts = Options::new();
	opts.optflag("h", "help", "Shows the help for this program.");
	opts.optopt("t", "test", "The test to execute", "SWITCH");
	
	let matches = match opts.parse(&args[1..]) {
		Ok(m) => { m },
		Err(e) => { panic!(e.to_string()) }
	};
	
	if matches.opt_present("h") {
		print_usage(&program, opts);
        return;
	}
	
	if matches.opt_present("t") {
		let tests = matches.opt_strs("t");
		if tests.len() == 0 {
			run_all();
		} else {
			for number in tests {
				match number.as_str() {
					"1" => { test_1_or_2(true); },
					"2" => { test_1_or_2(false); },
					"3" => { test_3_or_4(true); },
					"4" => { test_3_or_4(false); },
					_ => {
						println!("Unknown test '{}'", number);
					}
				}
			}
		}
	} else {
		run_all();
	}
}