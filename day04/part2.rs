use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let file = File::open("input.txt").unwrap();
	let reader = BufReader::new(file);
	let codes = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
	let mut present_codes = 0;
	let mut valid_codes = 0;
	let mut result = 0;

	for line in reader.lines() {
		let line = line.unwrap();
		if line.is_empty() {
			if present_codes == 7 && valid_codes == 7 {
				result += 1;
			}
			present_codes = 0;
			valid_codes = 0;
		}
		else{
			for index in 0..codes.len()-1 {
				if line.contains(codes[index]){
					present_codes += 1;
				}
			}
			for key_value in line.split_whitespace() {
				if check_validity(key_value, &codes) {
					valid_codes += 1;
				}
			}
		}
	}
	println!("{result}");
}

fn check_validity(key_value :&str, codes :&[&str]) -> bool {
	let key = key_value.split(":").next().unwrap();
	let value = key_value.split(":").last().unwrap();
	return if key == codes[0] {
		let value :i32 = value.parse().unwrap();
		if value >= 1920 && value <= 2002 {true} else {false}
	}
	else if key == codes[1] {
		let value :i32 = value.parse().unwrap();
		if value >= 2010 && value <= 2020 {true} else {false}
	}
	else if key == codes[2] {
		let value :i32 = value.parse().unwrap();
		if value >= 2020 && value <= 2030 {true} else {false}
	}
	else if key == codes[3] {
		if ! value.contains("cm") && ! value.contains("in") {
			return false;
		}
		let mut value = value.to_string();
		value.pop();
		let unit = value.pop().unwrap();
		let value :i32 = value.parse().unwrap();
		if (unit == 'c' && value >= 150 && value <= 193) || (unit == 'i' && value >= 59 && value <= 76) {true} else {false}
	}
	else if key == codes[4] {
		if value.contains("#") {
			return true;
		}
		false
	}
	else if key == codes[5] {
		let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
		for color in colors {
			if value == color {
				return true;
			}
		}
		return false;
	}
	else if key == codes[6] {
		if value.len() == 9 {
			return true;
		}
		return false;
	}
	else {false}

}
