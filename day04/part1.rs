use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let file = File::open("input.txt").unwrap();
	let reader = BufReader::new(file);
	let codes = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
	let mut present_codes = 0;
	let mut result = 0;

	for line in reader.lines() {
		let line = line.unwrap();
		if line.is_empty() {
			if present_codes == 7 {
				result += 1;
			}
			present_codes = 0;
		}
		else{
			for index in 0..codes.len()-1 {
				if line.contains(codes[index]){
					present_codes += 1;
				}
			}
		}
	}
	println!("{result}");
}
