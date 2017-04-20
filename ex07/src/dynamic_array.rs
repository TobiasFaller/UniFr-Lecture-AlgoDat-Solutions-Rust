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
		if self.size == 0 {
			return None;
		}
		
		let value = self.data.pop();
		self.size -= 1;
		
		println!("Size: {}, Capacity: {}", self.size, self.data.capacity());
		let capacity = self.data.capacity();
		if self.size * 3 <= capacity {
			self.resize(capacity / 2);
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
		
		println!("Resizing to {}", size);
		
		let mut vec: Vec<T> = Vec::new();
		vec.reserve_exact(size);
		
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
	let array = DynamicArray::<String>::new();
	
	assert_eq!(0, array.len());
	assert_eq!(2, array.capacity());
}

#[test]
fn test_append() {
	let mut array = DynamicArray::<String>::new();
	
	array.append("1".to_owned());
	
	let expected = vec!["1"];
	
	assert_eq!(expected.len(), array.len());
	assert_eq!(2, array.capacity());
	assert_eq!(expected, (0..expected.len()).map(|val| array.get(val).clone()).collect::<Vec<String>>());
	
	array.append("2".to_owned());
	array.append("3".to_owned());
	
	let expected = vec!["1", "2", "3"];
	assert_eq!(expected.len(), array.len());
	assert_eq!(4, array.capacity());
	assert_eq!(expected, (0..expected.len()).map(|val| array.get(val).clone()).collect::<Vec<String>>());
	
	array.append("4".to_owned());
	array.append("5".to_owned());
	
	let expected = vec!["1", "2", "3", "4", "5"];
	assert_eq!(expected.len(), array.len());
	assert_eq!(8, array.capacity());
	assert_eq!(expected, (0..expected.len()).map(|val| array.get(val).clone()).collect::<Vec<String>>());
}

#[test]
fn test_remove() {
	let mut array = DynamicArray::<String>::new();
	
	let data = (1..10).map(|i| format!("{}", i)).collect::<Vec<String>>();
	for str in data {
		array.append(str);
	}
	
	let expected = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
	
	assert_eq!(expected.len(), array.len());
	assert_eq!(16, array.capacity());
	assert_eq!(expected, (0..expected.len()).map(|val| array.get(val).clone()).collect::<Vec<String>>());
	
	array.remove();
	array.remove();
	array.remove();
	array.remove();
	
	let expected = vec!["1", "2", "3", "4", "5"];
	
	assert_eq!(expected.len(), array.len());
	assert_eq!(8, array.capacity());
	assert_eq!(expected, (0..expected.len()).map(|val| array.get(val).clone()).collect::<Vec<String>>());
	
	array.remove();
	array.remove();
	array.remove();
	
	let expected = vec!["1", "2"];
	
	assert_eq!(expected.len(), array.len());
	assert_eq!(4, array.capacity());
	assert_eq!(expected, (0..expected.len()).map(|val| array.get(val).clone()).collect::<Vec<String>>());
}

#[test]
fn test_append_remove() {
	let mut array = DynamicArray::<String>::new();
	
	assert_eq!(0, array.len());
	assert_eq!(2, array.capacity());
	
	array.append("1".to_owned());
	
	assert_eq!(1, array.len());
	assert_eq!(2, array.capacity());
	
	array.append("2".to_owned());
	
	assert_eq!(2, array.len());
	assert_eq!(2, array.capacity());
	
	array.remove();
	
	assert_eq!(1, array.len());
	assert_eq!(2, array.capacity());
	
	array.append("2".to_owned());
	
	assert_eq!(2, array.len());
	assert_eq!(2, array.capacity());
	
	array.append("3".to_owned());
	
	assert_eq!(3, array.len());
	assert_eq!(4, array.capacity());
	
	array.append("4".to_owned());
	array.append("5".to_owned());
	
	assert_eq!(5, array.len());
	assert_eq!(8, array.capacity());
	
	array.remove();
	
	assert_eq!(4, array.len());
	assert_eq!(8, array.capacity());
	
	array.remove();
	
	assert_eq!(3, array.len());
	assert_eq!(8, array.capacity());
	
	array.remove();
	
	assert_eq!(2, array.len());
	assert_eq!(4, array.capacity());
}