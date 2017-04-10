#![feature(box_syntax)]

extern crate time;

mod edit_distance;

use edit_distance::compute_ed_recursively;
use edit_distance::compute_ed_via_table;

use std::env;
use time::get_time;

fn print_usage(program: &str) {
    println!("Usage: {} <str1> <str2>", program);
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let program = &args[0];
	
	if args.len() != 3 {
		print_usage(program);
		return;
	}
	
	println!("Comparing '{}' with '{}'", &args[1], &args[2]);
	
	let start_time = get_time();
	let edit_dist_rec = compute_ed_recursively(&args[1], &args[2]);
	println!("Computed edit distance recursive {} in {} ms", edit_dist_rec,
		(get_time() - start_time).num_milliseconds());
	
	let start_time = get_time();
	let edit_dist_tab = compute_ed_via_table(&args[1], &args[2]);
	println!("Computed edit distance via table {} in {} ms", edit_dist_tab,
		(get_time() - start_time).num_milliseconds());
}