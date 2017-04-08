#![feature(box_syntax, box_patterns)]

extern crate time;

mod binary_search_tree;

use binary_search_tree::BinarySearchTree;

use time::get_time;

fn main() {
	let mut t: BinarySearchTree<i64, String> = BinarySearchTree::new();
	t.insert(5, "Hello World".to_owned());
	
	let value = t.lookup(5).unwrap();
	println!("{}: {}", value.0, value.1);
}