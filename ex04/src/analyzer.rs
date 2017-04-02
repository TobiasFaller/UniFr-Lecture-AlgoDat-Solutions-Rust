extern crate zip;

use std;

use std::result::Result;
use std::vec::Vec;
use std::string::String;

#[derive(Debug)]
pub enum Error {
	IoError(std::io::Error),
	ParseError(std::num::ParseIntError),
	ZipError(zip::result::ZipError)
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error::ParseError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Error {
        Error::ZipError(err)
    }
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			Error::IoError(ref err) => write!(f, "IO Error: {}", err),
			Error::ParseError(ref err) => write!(f, "Parse Error: {}", err),
			Error::ZipError(ref err) => write!(f, "Zip Error: {}", err)
		}
	}
}

impl std::error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::IoError(ref err) => err.description(),
			Error::ParseError(ref err) => err.description(),
			Error::ZipError(ref err) => err.description()
		}
	}
	
	fn cause(&self) -> Option<&std::error::Error> {
		match *self {
			Error::IoError(ref err) => Some(err),
			Error::ParseError(ref err) => Some(err),
			Error::ZipError(ref err) => Some(err)
		}
	}
}

pub fn read_info_from_file(name: &str) -> Result<Vec<(String, String)>, Error> {
	let file = try!(std::fs::File::open(name));
	let mut archive = try!(zip::ZipArchive::new(file));
	
	let mut cities: Vec<(String, String)> = Vec::new(); 
	for index in 0 .. archive.len() {
		let entry = try!(archive.by_index(index));
		let buf = std::io::BufReader::new(entry);
		try!(read_lines(buf, &mut cities));
	}
	
	Ok(cities)
}

fn read_lines<R: std::io::BufRead>(buf: R, cities: &mut Vec<(String, String)>) -> Result<(), Error> {
	for line_res in buf.lines() {
		let line = try!(line_res);
		
		let parts: Vec<&str> = line.split('\t').collect();
		if parts.len() < 15 {
			continue;
		}
		
		if parts[6] == "P" && try!(parts[14].parse::<i64>()) > 0 {
			cities.push((parts[1].to_owned(), parts[8].to_owned()));
		}
	}
	
	Ok(())
}

pub fn compute_most_frequent_city_by_sorting(cities: &mut Vec<(String, String)>) {
 
}