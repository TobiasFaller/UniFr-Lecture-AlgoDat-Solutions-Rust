use std::cell::UnsafeCell;
use std::cmp::Ord;
use std::rc::{Rc, Weak};
use std::mem;
use std::vec::Vec;
use std::fmt::Display;

pub struct PriorityQueue<K, T> where K: Ord + Clone + Display, T: Clone {
	elem: Vec<PriorityQueueItem<K, T>>
}

pub struct Handle<K, T> where K: Ord + Clone + Display, T: Clone {
	heap_index: Weak<QueueItemIndex>,
	pq: *const PriorityQueue<K, T>
}


struct PriorityQueueItem<K, T> where K: Ord + Clone + Display, T: Clone {
	heap_index: Rc<QueueItemIndex>,
	key: K,
	value: T
}
struct QueueItemIndex {
	index: UnsafeCell<usize>
}

impl<K, T> Handle<K, T> where K: Ord + Clone + Display, T: Clone {
	fn new(pq: *const PriorityQueue<K, T>, heap_index: Weak<QueueItemIndex>) -> Handle<K, T> {
		Handle {
			heap_index: heap_index,
			pq: pq
		}
	}
}

impl<'a, K: 'a, T: 'a> PriorityQueue<K, T> where K: Ord + Clone + Display, T: Clone {

	pub fn new() -> PriorityQueue<K, T> {
		PriorityQueue {
			elem: Vec::new()
		}
	}
	
	pub fn insert(&'a mut self, key: K, value: T) -> Handle<K, T> {
		let size = self.size();
		
		let queue_index = Rc::new(QueueItemIndex {
				index: UnsafeCell::new(size)
			});
		let handle_ptr = Rc::downgrade(&queue_index);
		
		// Create new item with last index in our list
		self.elem.push(PriorityQueueItem {
				key: key,
				value: value,
				heap_index: queue_index
			});
		
		// Repair heap upwards
		self.repair_heap_up(size);
		
		return Handle::new(self as *const PriorityQueue<K, T>, handle_ptr);
	}
	
	pub fn get_min(&self) -> Option<(K, T)> {
		let size = self.size();
		if size == 0 {
			return None;
		}
		
		let min = &self.elem[0];
		return Some((min.key.clone(), min.value.clone()));
	}

	pub fn size(&self) -> usize {
		self.elem.len()
	}
	
	pub fn delete_min(&mut self) {
		let size = self.size();
		if size == 0 {
			return;
		}
		
		self.elem.swap(0, size);
		self.elem.pop();
		
		// Repair heap downwards
		self.repair_heap_down(0);
	}
	
	pub fn pop(&mut self) -> Option<(K, T)> {
		let size = self.size();
		if size == 0 {
			return None;
		}

		let item;
		if size == 1 {
			item = self.elem.remove(0);
		} else {
			item = self.elem.swap_remove(0);

			unsafe {
				let mut index_root = self.elem[0].heap_index.index.get();
				*index_root = 0;
			}

			// Repair heap downwards
			self.repair_heap_down(0);
		}

		return Some((item.key, item.value));
	}
	
	pub fn change_key(&mut self, handle: &Handle<K, T>, key: K) -> bool {
		if handle.pq != self {
			return false;
		}
		
		let index: usize;
		match handle.heap_index.upgrade() {
			None => {
				return false;
			},
			Some(ptr) => {
				unsafe {
					let cell_value = ptr.index.get();
					index = *cell_value.clone();
				}
			}
		}

		let new_key = key;
		let mut k = new_key.clone();
		{
			mem::swap(&mut self.elem[index].key, &mut k);
		}

		if k > new_key {
			self.repair_heap_up(index);
		} else if k < new_key {
			self.repair_heap_down(index);
		}

		return true;
	}
	
	pub fn remove(&mut self, handle: &Handle<K, T>) -> bool {
		if handle.pq != self {
			return false;
		}
		
		let index: usize;
		match handle.heap_index.upgrade() {
			None => {
				return false;
			},
			Some(ptr) => {
				unsafe {
					let cell_value = ptr.index.get();
					index = *cell_value.clone();
				}
			}
		}
		
		let last_index = self.elem.len() - 1;
		swap(&mut self.elem, index, last_index);

		self.repair_heap_down(index);
		
		// Actually remove element
		self.elem.pop();

		return true;
	}
	
	fn repair_heap_up(&mut self, index: usize) {
		let mut index = index.clone();

		while index > 0 {
			let parent: usize = (index - 1) / 2;
			
			if self.elem[index].key >= self.elem[parent].key {
				return;
			}
			
			swap(&mut self.elem, parent, index);
			
			index = parent;
		}
	}
	
	fn repair_heap_down(&mut self, index: usize) {
		let size = self.size();
		let elem: &mut Vec<PriorityQueueItem<K, T>> = &mut self.elem;

		let mut index = index.clone();
		while index * 2 + 1 < size {
			let child_left = index * 2 + 1;
			let child_right = index * 2 + 2;

			let mut swap_index = index;

			// Choose the element we want to swap with
			if child_left < size && elem[child_left].key < elem[swap_index].key {
				swap_index = child_left;
			}
			if child_right < size && elem[child_right].key < elem[swap_index].key {
				swap_index = child_right;
			}

			if swap_index == index {
				return;
			}

			swap(elem, swap_index, index);
			
			index = swap_index;
		}
	}
}

fn swap<K, T>(elem: &mut Vec<PriorityQueueItem<K, T>>, a: usize, b: usize) where K: Ord + Clone + Display, T: Clone {
	elem.swap(a, b);
	
	let mut index_a = elem[a].heap_index.index.get();
	let mut index_b = elem[b].heap_index.index.get();
	
	unsafe {
		*index_a = a;
		*index_b = b;
	}
}

#[test]
fn test_general() {
	let mut q = PriorityQueue::new();
	q.insert(103, "503");
	q.insert(100, "500");
	q.insert(102, "502");
	q.insert(101, "501");
	
	let expected = vec![(100, "500"), (101, "501"), (102, "502"), (103, "503")];
	
	for val in expected {
		match q.pop() {
			None => {
				assert!(false);
			},
			Some(q_val) => {
				assert_eq!(val, q_val);
			}
		}
	}
}

#[test]
fn test_remove() {
	let mut q = PriorityQueue::new();
	q.insert(103, "503");
	q.insert(100, "500");
	let item = q.insert(102, "502");
	q.insert(101, "501");
	q.insert(104, "504");
	
	q.remove(&item);
	
	let expected = vec![(100, "500"), (101, "501"), (103, "503"), (104, "504")];
	
	for val in expected {
		match q.pop() {
			None => {
				assert!(false);
			},
			Some(q_val) => {
				assert_eq!(val, q_val);
			}
		}
	}
}

#[test]
fn test_change_priority() {
	let mut q = PriorityQueue::new();
	q.insert(103, "503");
	q.insert(100, "500");
	let item = q.insert(102, "502");
	q.insert(101, "501");
	q.insert(104, "504");
	
	q.change_key(&item, 105);
	
	q.insert(106, "506");
	q.insert(102, "502");
	
	let expected = vec![(100, "500"), (101, "501"), (102, "502"), (103, "503"), (104, "504"), (105, "502"), (106, "506")];
	
	for val in expected {
		match q.pop() {
			None => {
				assert!(false);
			},
			Some(q_val) => {
				assert_eq!(val, q_val);
			}
		}
	}
}