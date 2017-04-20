use std::borrow::BorrowMut;
use std::usize;
use std::fmt::{Display, Formatter, Error as FmtError};

pub struct BinarySearchTree<Key, Value> {
	root: Option<Box<Node<Key, Value>>>,
	depth: usize
}

struct Node<Key, Value> {
	key: Key,
	value: Value,
	parent: Option<*const Option<Box<Node<Key, Value>>>>,
	left: Option<Box<Node<Key, Value>>>,
	right: Option<Box<Node<Key, Value>>>
}

impl<Key, Value> Node<Key, Value>
where Key: PartialOrd + PartialEq + Display, Value: Display {
	
	fn new(key: Key, value: Value, parent: Option<*const Option<Box<Node<Key, Value>>>>) -> Node<Key, Value> {
		Node {
			key: key,
			value: value,
			parent: parent,
			left: None,
			right: None
		}
	}
}

impl<Key, Value> BinarySearchTree<Key, Value>
where Key: PartialOrd + PartialEq + Display, Value: Display {
	
	pub fn new() -> BinarySearchTree<Key, Value> {
		BinarySearchTree {
			root: None,
			depth: 0
		}
	}
	
	pub fn depth(&self) -> usize {
		self.depth
	}
	
	pub fn insert(&mut self, key: Key, value: Value) {
		let mut current: *mut Option<Box<Node<Key, Value>>> = self.root.borrow_mut();
		let mut parent: Option<*const Option<Box<Node<Key, Value>>>> = None;
		let mut depth: usize = 1;
		
		loop {
			unsafe {
				match *current {
					None => {
						*current = Some(box Node::new(key, value, parent));
						if depth > self.depth {
							self.depth = depth;
						} 
						return;
					},
					Some(ref mut inner) => {
						depth += 1;
						if inner.key == key {
							inner.value = value;
							return;
						}
						
						parent = Some(current);
						if key < inner.key {
							current = inner.left.borrow_mut();
						} else {
							current = inner.right.borrow_mut();
						}
					}
				};
			}
		}
	}
	
	pub fn lookup(&self, key: Key) -> Option<(Key, &Value)> {
		let mut current: *const Option<Box<Node<Key, Value>>> = &self.root;
		
		loop {
			unsafe {
				match *current {
					None => {
						return None;
					},
					Some(box ref inner) => {
						if inner.key == key {
							return Some((key, &inner.value));
						} else if key < inner.key {
							current = &inner.left;
						} else {
							current = &inner.right;
						}
					}
				}
			}
		}
	}
}

impl<Key, Value> Display for BinarySearchTree<Key, Value>
where Key: PartialOrd + PartialEq + Display, Value: Display {

	fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
		if let None = self.root {
			return fmt.write_str("None");
		}

		let mut current: *const Option<Box<Node<Key, Value>>> = &self.root;
		let mut current_part = 0_usize;

		let mut last: *const Option<Box<Node<Key, Value>>> = &self.root;
		let mut last_part = 0_usize;

		loop {
			unsafe {
				match *current {
					None => {
						try!(write!(fmt, "None"));

						// Go back one level up (use the saved last node)
						current = last;
						current_part = last_part + 1;
					},
					Some(box ref inner) => {
						// Did we finish the complete tree?
						if current == &self.root && current_part == 2 {
							// Finish the complete tree
							return fmt.write_str("]");
						}

						match current_part {
							0 => { // Start of new element
								try!(write!(fmt, "[({}, {}), left: ", &inner.key, &inner.value));

								// Go down left and print left child
								last = current;
								last_part = current_part;
								current = &inner.left;
								current_part = 0;
							},
							1 => { // Print separator
								try!(write!(fmt, ", right: "));

								// Go down right and print right child
								last = current;
								last_part = current_part;
								current = &inner.right;
								current_part = 0;
							},
							_ => { // Print closing bracket
								try!(write!(fmt, "]"));

								// Are we printing the right child?
								if let Some(parent_ptr) = inner.parent {
									if let Some(box ref parent) = *parent_ptr {
										current_part = if current == &parent.right { 2 } else { 1 };
									} else {
										panic!("Tree is corrupted or not traversed correctly!");
									}

									// Go back up
									current = parent_ptr;
								} else {
									panic!("Tree is corrupted or not traversed correctly!");
								}
							}
						}
					}
				}
			}
		}
	}
}

#[test]
fn test_insert() {
	let mut tree = BinarySearchTree::<usize, String>::new();
	tree.insert(105, "505".to_owned());
	tree.insert(108, "508".to_owned());
	tree.insert(102, "502".to_owned());
	tree.insert(101, "501".to_owned());
	tree.insert(100, "500".to_owned());

	assert_eq!("[(105, 505), left: [(102, 502), left: [(101, 501), left: [(100, 500), left: None, right: None], \
			right: None], right: None], right: [(108, 508), left: None, right: None]]",
		format!("{}", tree));
}

#[test]
fn test_lookup() {
	let mut tree = BinarySearchTree::<usize, String>::new();
	tree.insert(105, "505".to_owned());
	tree.insert(108, "508".to_owned());
	tree.insert(102, "502".to_owned());
	tree.insert(101, "501".to_owned());
	tree.insert(100, "500".to_owned());

	assert_eq!(None, tree.lookup(110));
	assert_eq!(Some((100, &"500".to_owned())), tree.lookup(100));
}

#[test]
fn test_lookup2() {
	let mut tree = BinarySearchTree::<usize, String>::new();
	tree.insert(105, "505".to_owned());
	tree.insert(100, "501".to_owned()); // Will be replaced
	tree.insert(100, "500".to_owned());
	tree.insert(107, "507".to_owned());
	tree.insert(103, "503".to_owned());

	assert_eq!(Some((103, &"503".to_owned())), tree.lookup(103));
	assert_eq!(None, tree.lookup(108));
}