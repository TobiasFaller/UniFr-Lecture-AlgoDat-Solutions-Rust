extern crate zip;

use self::zip::ZipArchive;
use self::zip::result::ZipError;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IOError};
use std::num::ParseIntError;
use std::result::Result;
use std::string::String;
use std::vec::Vec;

#[derive(Debug)]
pub enum Error {
	IoError(IOError),
	ParseError(ParseIntError),
	ZipError(ZipError)
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error::ParseError(err)
    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Error {
        Error::IoError(err)
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(err: ZipError) -> Error {
        Error::ZipError(err)
    }
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		match *self {
			Error::IoError(ref err) => write!(f, "IO Error: {}", err),
			Error::ParseError(ref err) => write!(f, "Parse Error: {}", err),
			Error::ZipError(ref err) => write!(f, "Zip Error: {}", err)
		}
	}
}

impl StdError for Error {
	fn description(&self) -> &str {
		match *self {
			Error::IoError(ref err) => err.description(),
			Error::ParseError(ref err) => err.description(),
			Error::ZipError(ref err) => err.description()
		}
	}
	
	fn cause(&self) -> Option<&StdError> {
		match *self {
			Error::IoError(ref err) => Some(err),
			Error::ParseError(ref err) => Some(err),
			Error::ZipError(ref err) => Some(err)
		}
	}
}

pub fn read_info_from_file(name: &str) -> Result<Vec<(String, String)>, Error> {
	let file = File::open(name)?;
	let mut archive = ZipArchive::new(file)?;
	
	let mut cities: Vec<(String, String)> = Vec::new();
	cities.reserve(20000);
	
	for index in 0 .. archive.len() {
		let entry = archive.by_index(index)?;
		let buf = BufReader::new(entry);
		read_lines(buf, &mut cities)?;
	}
	
	Ok(cities)
}

fn read_lines<R: BufRead>(buf: R, cities: &mut Vec<(String, String)>) -> Result<(), Error> {
	for line_res in buf.lines() {
		let line = line_res?;
		
		let parts: Vec<&str> = line.split('\t').collect();
		if parts.len() < 15 {
			continue;
		}
		
		if parts[6] == "P" && parts[14].parse::<i64>()? > 0 {
			cities.push((parts[1].to_owned(), parts[8].to_owned()));
		}
	}
	
	Ok(())
}

pub fn compute_most_frequent_city_by_sorting(mut cities: Vec<(String, String)>)
		-> Vec<(String, usize)> {
	let length = cities.len();
	if length == 0 {
		return Vec::new();
	}
	
	cities.sort_by(|a, b| a.0.cmp(&b.0));
	
	let mut names: Vec<(String, usize)> = Vec::new();
	let mut last_name: &str = &cities[0].0;
	let mut count: usize = 1;
	
	for city in cities.iter().skip(1) {
		if city.0 == last_name {
			count += 1;
		} else {
			names.push((last_name.to_owned(), count));
			last_name = &city.0;
			count = 1
		}
	}
	
	names.push((cities[length - 1].0.to_owned(), count));
	names.sort_by(|a, b| b.1.cmp(&a.1)
		.then(b.0.cmp(&a.0).reverse()));
	
	return names;
}

pub fn compute_most_frequent_city_by_map(cities: &Vec<(String, String)>) -> Vec<(String, usize)> {
	let length = cities.len();
	if length == 0 {
		return Vec::new();
	}
	
	let mut map: HashMap<&str, usize> = HashMap::new();
	for city in cities {
		let name: &str = &city.0;
		
		match map.entry(name) {
			Entry::Occupied(mut o) => {
				let new_value = o.get() + 1;
				o.insert(new_value);
			},
			Entry::Vacant(v) => {
				v.insert(1);
			}
		}
	}
	
	let mut names: Vec<(String, usize)> = Vec::new();
	for (name, count) in map.drain() {
		names.push((name.to_owned(), count));
	}
	names.sort_by(|a, b| b.1.cmp(&a.1)
		.then(b.0.cmp(&a.0).reverse()));
	
	return names;
}

pub fn compute_most_frequent_city_by_sorting_in_de(mut cities: Vec<(String, String)>)
		-> Vec<(String, usize)> {
	let length = cities.len();
	if length == 0 {
		return Vec::new();
	}
	
	cities.sort_by(|a, b| a.0.cmp(&b.0));
	
	let mut names: Vec<(String, usize)> = Vec::new();
	let mut last_name: &str = &cities[0].0;
	let mut count: usize = 1;
	let mut in_de: bool = &cities[0].0 == "DE";
	
	for city in cities.iter().skip(1) {
		if city.0 == last_name {
			count += 1;
			in_de |= city.1 == "DE";
		} else {
			if in_de {
				names.push((last_name.to_owned(), count));
			}
			
			last_name = &city.0;
			count = 1;
			in_de = city.1 == "DE";
		}
	}
	
	if in_de || cities[length - 1].1 == "DE" {
		names.push((cities[length - 1].0.to_owned(), count));
	}
	
	names.sort_by(|a, b| b.1.cmp(&a.1)
		.then(b.0.cmp(&a.0).reverse()));
	
	return names;
}

pub fn compute_most_frequent_city_by_map_in_de(cities: &Vec<(String, String)>)
		-> Vec<(String, usize)> {
	let length = cities.len();
	if length == 0 {
		return Vec::new();
	}
	
	let mut map: HashMap<&str, (usize, bool)> = HashMap::new();
	for city in cities {
		let name: &str = &city.0;
		
		match map.entry(name) {
			Entry::Occupied(mut o) => {
				let old_value = o.get_mut();
				old_value.0 += 1;
				old_value.1 |= city.1 == "DE";
			},
			Entry::Vacant(v) => {
				v.insert((1, city.1 == "DE"));
			}
		}
	}
	
	let mut names: Vec<(String, usize)> = Vec::new();
	for (name, (count, in_de)) in map.drain() {
		if in_de {
			names.push((name.to_owned(), count));
		}
	}
	
	names.sort_by(|a, b| b.1.cmp(&a.1)
		.then(b.0.cmp(&a.0).reverse()));
	
	return names;
}

#[test]
fn test_read() {
	if let Ok(data) = read_info_from_file("test_data.zip") {
		assert_eq!(16, data.len());
	} else {
		assert!(false);
	}
}

#[test]
fn test_analyzer() {
	if let Ok(data) = read_info_from_file("test_data.zip") {
		let cities_map = compute_most_frequent_city_by_map(&data);
		let cities_sort = compute_most_frequent_city_by_sorting(data);
		
		let expected = vec![
			("Köln".to_owned(), 4_usize),
			("Freiburg".to_owned(), 3_usize),
			("Fruiburg".to_owned(), 3_usize)
		];
		
		assert_eq!(expected, cities_map[..3].to_vec());
		assert_eq!(expected, cities_sort[..3].to_vec());
	} else {
		assert!(false);
	}
}

#[test]
fn test_analyzer_de() {
	if let Ok(data) = read_info_from_file("test_data.zip") {
		let cities_map = compute_most_frequent_city_by_map_in_de(&data);
		let cities_sort = compute_most_frequent_city_by_sorting_in_de(data);
		
		// Now 'Fruiburg' is outnumbered by 'Friburg'
		let expected = vec![
			("Köln".to_owned(), 4_usize),
			("Freiburg".to_owned(), 3_usize),
			("Friburg".to_owned(), 2_usize)
		];
		
		assert_eq!(expected, cities_map[..3].to_vec());
		assert_eq!(expected, cities_sort[..3].to_vec());
	} else {
		assert!(false);
	}
}