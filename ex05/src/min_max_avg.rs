use std::default::Default;
use std::iter::{FromIterator, IntoIterator};
use std::ops::{Add, Mul, Div};
use std::cmp::Ordering::{Less, Greater};

pub struct MinMaxAvg<T> where T: Add<T, Output=T> + Mul<f64, Output = T> + Div<f64, Output = T> + PartialOrd + Clone {
	// min, max, avg
	values: Option<(T, T, T)>,
    len: usize
}

impl<T> MinMaxAvg<T> where T: Add<T, Output=T> + Mul<f64, Output = T> + Div<f64, Output = T> + PartialOrd + Clone {
	fn new() -> MinMaxAvg<T> {
		Default::default()
	}
	
	pub fn min(&self) -> Option<T> {
		match self.values {
			Some(ref value) => {
				return Some(value.1.clone());
			},
			None => {
				return None;
			}
		}
	}
	
	pub fn max(&self) -> Option<T> {
		match self.values {
			Some(ref value) => {
				return Some(value.2.clone());
			},
			None => {
				return None;
			}
		}
	}
	
	pub fn mean(&self) -> Option<T> {
		match self.values {
			Some(ref value) => {
				return Some(value.0.clone())
			},
			None => {
				return None;
			}
		}
	}
	
	pub fn len(&self) -> usize {
		return self.len;
	}
	
	pub fn get(&self) -> Option<(T, T, T)> {
		match self.values {
			Some (ref value) => {
				return Some((value.0.clone(), value.1.clone(), value.2.clone()));
			},
			None => {
				return None;
			}
		}
	}
	
	pub fn add(&mut self, sample: T) {
		match self.values {
			Some(ref mut value) => {
				let calc_min;
				let calc_max;
				let calc_avg;
				
				{
					let mut new_min = &value.0;
					let mut new_max = &value.1;
					
					match new_min.partial_cmp(&sample) {
						Some(ordering) => {
							if ordering == Greater {
								new_min = &sample;
							}
						},
						_ => { }
					}
					
					match new_max.partial_cmp(&sample) {
						Some(ordering) => {
							if ordering == Less {
								new_max = &sample;
							}
						},
						_ => { }
					}
					
					calc_min = new_min.clone();
					calc_max = new_max.clone();
					calc_avg = (value.2.clone() * self.len as f64 + (&sample).clone()) / (self.len as f64 + 1.0);
				}
				
				print!("New value: {:?} {:?} {:?}", &calc_min, &calc_max, &calc_avg);
				
				value.0 = calc_min;
				value.1 = calc_max;
				value.2 = calc_avg;
				self.len += 1;
			},
			None => {
				self.values = Some((sample.clone(), sample.clone(), sample.clone()));
			}
		}
	}
}

impl<T> Default for MinMaxAvg<T> where T: Add<T, Output=T> + Mul<f64, Output = T> + Div<f64, Output = T> + PartialOrd + Clone {
    fn default() -> MinMaxAvg<T> {
        MinMaxAvg {
        	values: None,
		    len: 0
        }
    }
}

impl<'a, T: 'a> FromIterator<&'a T> for MinMaxAvg<T> where T: Add<T, Output=T> + Mul<f64, Output = T> + Div<f64, Output = T> + PartialOrd + Copy {
    fn from_iter<I: IntoIterator<Item=&'a T>>(it: I) -> MinMaxAvg<T> {
        let mut v = MinMaxAvg::<T>::new();
        v.extend(it);
        return v;
    }
}

impl<'a, T: 'a> Extend<&'a T> for MinMaxAvg<T> where T: Add<T, Output=T> + Mul<f64, Output = T> + Div<f64, Output = T> + PartialOrd + Copy {
    fn extend<I: IntoIterator<Item=&'a T>>(&mut self, it: I) {
        for sample in it {
            self.add(sample.clone());
        }
    }
}

/*impl<T: Add + Mul + Div> Commute for MinMaxAvg<T> {
	fn merge(&mut self, v: MinMaxAvg) {
		self.avg = self.avg / self.num + v.avg / self.num;
		self.min = self.min.min(v.min);
		self.max = self.max.max(v.max);
		self.num += v.num;
	}
}*/