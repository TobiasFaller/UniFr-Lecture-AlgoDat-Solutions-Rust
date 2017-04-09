use std::borrow::BorrowMut;
use std::usize;

pub struct BinarySearchTree<K, T> where K: PartialOrd + PartialEq {
	root: Option<Box<Node<K, T>>>,
	depth: usize
}

struct Node<K, T> where K: PartialOrd + PartialEq {
	key: K,
	value: T,
	left: Option<Box<Node<K, T>>>,
	right: Option<Box<Node<K, T>>>
}

impl<K, T> Node<K, T> where K: PartialOrd + PartialEq {
	
	fn new(key: K, value: T) -> Node<K, T> {
		Node {
			key: key,
			value: value,
			left: None,
			right: None
		}
	}
	
}

#[allow(dead_code)]
impl<K, T> BinarySearchTree<K, T> where K: PartialOrd + PartialEq {
	
	pub fn new() -> BinarySearchTree<K, T> {
		BinarySearchTree {
			root: None,
			depth: 0
		}
	}
	
	pub fn depth(&self) -> usize {
		self.depth
	}
	
	pub fn insert(&mut self, key: K, value: T) {
		let mut current: *mut Option<Box<Node<K, T>>> = self.root.borrow_mut();
		let mut depth: usize = 1;
		
		loop {
			unsafe {
				match *current {
					None => {
						*current = Some(box Node::new(key, value));
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
						
						if inner.key < key {
							current = inner.left.borrow_mut();
						} else {
							current = inner.right.borrow_mut();
						}
					}
				};
			}
		}
	}
	
	pub fn lookup(&self, key: K) -> Option<(K, &T)> {
		let mut current: *const Option<Box<Node<K, T>>> = &self.root;
		
		loop {
			unsafe {
				match *current {
					None => {
						return None;
					},
					Some(box ref inner) => {
						if inner.key == key {
							return Some((key, &inner.value));
						} else if inner.key < key {
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