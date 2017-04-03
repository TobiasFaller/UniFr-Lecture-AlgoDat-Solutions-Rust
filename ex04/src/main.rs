extern crate getopts;
extern crate time;

mod analyzer;

use analyzer::read_info_from_file;
use analyzer::compute_most_frequent_city_by_sorting;
use analyzer::compute_most_frequent_city_by_map;

use getopts::Options;
use std::env;

const FILE: &'static str = "allCountries.zip";

fn print_usage(program: &str, opts: Options) {
    println!("{}", opts.usage(&format!("Usage: {} [options]", program)));
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let program = &args[0];
	
	let mut opts = Options::new();
	opts.optflag("h", "help", "Shows the help for this program.");
	opts.optopt("f", "file", "The file to read from.", "FILE");
	
	let matches = match opts.parse(&args[1..]) {
		Ok(m) => { m },
		Err(e) => { panic!(e.to_string()) }
	};
	
	if matches.opt_present("h") {
		print_usage(&program, opts);
        return;
	}
	
	let mut file: String = FILE.to_owned();
	
	if matches.opt_present("f") {
		match matches.opt_str("f") {
			None => { },
			Some(name) => {
				file = name;
			}
		}
	}
	
	println!("Loading input file");
	
	let mut start_time = time::get_time();
	match read_info_from_file(&file) {
		Ok(cities) => {
			print_time("Loaded city names in ", "", time::get_time() - start_time);
			println!();
			
			// --------------- List --------------------
			
			let city_set = cities.clone();
			
			start_time = time::get_time();
			let names = compute_most_frequent_city_by_sorting(city_set);
			print_time("Listed city names by sorting in ", "", time::get_time() - start_time);
			
			for name in names.iter().take(3).enumerate() {
				println!("{}: {} with {} occurences", name.0, (name.1).0, (name.1).1);
			}
			
			println!();
			
			// --------------- Map --------------------
			
			let city_set = cities.clone();
			
			start_time = time::get_time();
			let names = compute_most_frequent_city_by_map(&city_set);
			print_time("Listed city names by map in ", "", time::get_time() - start_time);
			
			for name in names.iter().take(3).enumerate() {
				println!("{}: {} with {} occurences", name.0, (name.1).0, (name.1).1);
			}
		},
		Err(error) => {
			println!("{}", error);
		}
	}
}

fn print_time(prefix: &str, suffix: &str, time: time::Duration) {
	let minutes = time.num_minutes() + time.num_hours() * 60;
	let seconds = time.num_seconds() - minutes * 60;
	let milli_seconds = time.num_milliseconds() - seconds * 1000 - minutes * 60 * 1000;
	
	println!("{}{} min {} s {} ms{}", prefix, minutes, seconds, milli_seconds, suffix);
}