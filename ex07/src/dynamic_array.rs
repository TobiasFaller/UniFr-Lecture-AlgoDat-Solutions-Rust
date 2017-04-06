use std::usize;
use std::mem::swap;
use std::vec::Vec;

pub struct DynamicArray<T> {
	data: Vec<T>,
	size: usize
}

impl<T> DynamicArray<T> {
	
	pub fn new() -> DynamicArray<T> {
		let mut a = DynamicArray {
			data: Vec::new(),
			size: 0
		};
		a.resize(10);
		return a;
	}
	
	pub fn len(&self) -> usize {
		return self.size;
	}
	
	pub fn capacity(&self) -> usize {
		return self.data.capacity();
	}
	
	pub fn append(&mut self, value: T) {
		let size = self.size;
		
		if size == self.data.capacity() {
			self.resize(2 * size);
		}
		
		self.data.push(value);
		self.size += 1;
	}
	
	pub fn remove(&mut self) -> Option<T> {
		let size = self.size;
		
		if size == 0 {
			return None;
		}
		
		let value = self.data.pop();
		
		if size <= self.data.capacity() / 3 {
			self.resize(3 * size / 2);
		}
		
		return value;
	}
	
	fn resize(&mut self, size: usize) {
		let mut size = size;
		if size < 10 {
			size = 10;
		}
		
		if size == self.data.capacity() {
			return;
		}
		
		let mut vec: Vec<T> = Vec::with_capacity(size);
		swap(&mut vec, &mut self.data);
		
		for item in vec {
			self.data.push(item);
		}
	}
	
}