use std::boxed::Box;
use std::cmp::{Ord, Ordering};
use std::rc::{Rc, Weak};
use std::mem;
use std::vec::Vec;

struct PriorityQueueItem<K, T> where K: Ord + Clone, T: Clone {
	heap_index: usize,
	
	key: K,
	value: T
}

impl<K, T> PriorityQueueItem<K, T> where K: Ord + Clone, T: Clone {
	fn new(key: K, value: T) -> PriorityQueueItem<K, T> {
		PriorityQueueItem {
			heap_index: 0,
			
			key: key,
			value: value
		}
	}
}

impl<K, T> PartialOrd for PriorityQueueItem<K, T> where K: Ord + Clone, T: Clone {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		return Some(self.key.cmp(&other.key));
	}
}

impl<K, T> Ord for PriorityQueueItem<K, T> where K: Ord + Clone, T: Clone {
	fn cmp(&self, other: &Self) -> Ordering {
		return self.key.cmp(&other.key);
	}
}

impl<K, T> PartialEq for PriorityQueueItem<K, T> where K: Ord + Clone, T: Clone {
	fn eq(&self, other: &Self) -> bool {
		self.key == other.key
	}
}

impl<K, T> Eq for PriorityQueueItem<K, T> where K: Ord + Clone, T: Clone { }

pub struct Handle<K, T> where K: Ord + Clone, T: Clone {
	item: Option<Weak<PriorityQueueItem<K, T>>>,
	pq: *const PriorityQueue<K, T>
}

impl<K, T> Handle<K, T> where K: Ord + Clone, T: Clone {
	
	fn new(pq: *const PriorityQueue<K, T>, item: Option<Weak<PriorityQueueItem<K, T>>>) -> Handle<K, T> {
		Handle {
			item: item,
			pq: pq
		}
	}
	
}

pub struct PriorityQueue<K, T> where K: Ord + Clone, T: Clone {
	elem: Vec<Rc<PriorityQueueItem<K, T>>>
}

impl<'a, K: 'a, T: 'a> PriorityQueue<K, T> where K: Ord + Clone, T: Clone {

	pub fn new() -> PriorityQueue<K, T> {
		PriorityQueue {
			elem: Vec::new()
		}
	}
	
	pub fn insert(&'a mut self, key: K, value: T) -> Handle<K, T> {
		let size = self.size();
		
		// Create new item with last index in our list
		let mut new_item = PriorityQueueItem::new(key, value);
		new_item.heap_index = size;
		
		let ptr = Rc::new(new_item);
		let handle_ptr = Rc::downgrade(&ptr);
		self.elem.push(ptr);
		
		// Repair heap upwards
		self.repair_heap_up(size);
		
		return Handle::new(self as *const PriorityQueue<K, T>, Some(handle_ptr));
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

			// Repair heap downwards
			self.repair_heap_down(0);
		}

		if let Ok(item) = Rc::try_unwrap(item) {
			return Some((item.key, item.value));
		} else {
			return None;
		}
	}
	
	pub fn change_key(&mut self, handle: &mut Handle<K, T>, key: K) -> bool {
		if handle.pq != self {
			return false;
		}

		let index: usize;
		{
			// This scope will destroy the weak reference
			let mut weak_ptr = None;
			mem::swap(&mut handle.item, &mut weak_ptr);

			match weak_ptr {
				None => {
					panic!("This method is not thread safe!");
				},
				Some(ptr) => {
					match ptr.upgrade() {
						None => {
							return false;
						},
						Some(ptr) => {
							index = ptr.as_ref().heap_index;
						}
					}
				}
			}
		}

		let mut old_key = key.clone();
		let new_key = key;
		{
			println!("Strong: {}, Weak: {}", Rc::strong_count(&self.elem[index]), Rc::weak_count(&self.elem[index]));
			if let Some(item) = Rc::get_mut(&mut self.elem[index]) {
				mem::swap(&mut old_key, &mut item.key);
			} else {
				panic!("Cannot modify because of existing references!");
			}

			// Fix removed weak pointer
			let mut weak_ptr = Some(Rc::downgrade(&self.elem[index]));
			mem::swap(&mut handle.item, &mut weak_ptr);
		}

		if old_key > new_key {
			self.repair_heap_up(index);
		} else if old_key < new_key {
			self.repair_heap_down(index);
		}

		return true;
	}
	
	fn repair_heap_up(&mut self, index: usize) {
		let mut index = index.clone();

		while index > 0 {
			let parent: usize = (index - 1) / 2;
			
			if self.elem[index] >= self.elem[parent] {
				return;
			}
			
			index = parent;
		}
	}
	
	fn repair_heap_down(&mut self, index: usize) {
		let size = self.size();
		let elem: &mut Vec<Rc<PriorityQueueItem<K, T>>> = &mut self.elem;

		let mut index = index.clone();
		while index * 2 + 1 < size {
			let child_left = index * 2 + 1;
			let child_right = index * 2 + 2;

			let mut swap_index = index;

			// Choose the element we want to swap with
			if child_left < size && elem[child_left] < elem[swap_index] {
				swap_index = child_left;
			}
			if child_right < size && elem[child_right] < elem[swap_index] {
				swap_index = child_right;
			}

			if swap_index == index {
				return;
			}

			elem.swap(swap_index, index);
			index = swap_index;
		}
	}
}