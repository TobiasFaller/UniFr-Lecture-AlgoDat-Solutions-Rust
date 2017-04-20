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
		a.resize(2);
		return a;
	}
	
	pub fn capacity(&self) -> usize {
		return self.data.capacity();
	}
	
	pub fn len(&self) -> usize {
		return self.data.len();
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
		if size < 1 {
			size = 1;
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
	
	fn get(&self, index: usize) -> &T {
		return &self.data[index];
	}
}

#[test]
fn test_fresh_list() {
	let array = DynamicArray::<&str>::new();
	
	assert_eq!(0, array.len());
	assert_eq!(2, array.capacity());
}

#[test]
fn test_append() {
	let mut array = DynamicArray::<&str>::new();
	
	array.append("1");
	
	let expected = vec!["1"];
	
	assert_eq!(expected.len(), array.len());
	assert_eq!(2, array.capacity());
	assert_eq!(expected.iter().collect::<Vec<&&str>>(),
		(0..expected.len()).map(|val| array.get(val)).collect::<Vec<&&str>>());
	
	array.append("2");
	array.append("3");
	
	let expected = vec!["1", "2", "3"];
	assert_eq!(expected.len(), array.len());
	assert_eq!(4, array.capacity());
	assert_eq!(expected.iter().collect::<Vec<&&str>>(),
		(0..expected.len()).map(|val| array.get(val)).collect::<Vec<&&str>>());
	
	array.append("4");
	array.append("5");
	
	let expected = vec!["1", "2", "3", "4", "5"];
	assert_eq!(expected.len(), array.len());
	assert_eq!(8, array.capacity());
	assert_eq!(expected.iter().collect::<Vec<&&str>>(),
		(0..expected.len()).map(|val| array.get(val)).collect::<Vec<&&str>>());
}