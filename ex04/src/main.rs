mod analyzer;

use analyzer::read_info_from_file;

const FILE: &'static str = "allCountries.zip";

fn main() {
	match read_info_from_file(FILE) {
		Ok(data) => {
			println!("{:?}", data);
		}
		Err(error) => {
			println!("{}", error);		
		}
	}
}