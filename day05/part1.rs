use std::fs::File;
use std::io::{BufRead, BufReader};

struct Range {
	min: i32,
	max: i32
}

fn main() {
	let file = File::open("input.txt").unwrap();
	let reader = BufReader::new(file);

	let mut result = 0;
	let mut row = 0;
	let mut column = 0;
	let mut row_range = Range {min: 0, max: 127};
	let mut column_range = Range {min: 0, max: 7};

	for line in reader.lines() {
		let mut chars: Vec<char> = line.unwrap().chars().collect();
		row = cordinate(&chars, 0, 7, &mut row_range, 'F', 'B');
		column = cordinate(&chars, 7, 10, &mut column_range, 'L', 'R');
		let possible_result = row * 8 + column;
		if possible_result > result {
			result = possible_result;
		}
		row_range.min = 0;
		row_range.max = 127;
		column_range.min = 0;
		column_range.max = 7;
	}
	println!("{}", result);
}

fn cordinate (chars: &Vec<char>, start_index: i32, end_index: i32, range: &mut Range, lower_half: char, upper_half: char) -> i32 {
	let mut direction = 0;
	for i in start_index..end_index {
		if chars[i as usize] == lower_half {
			if range.max - range.min == 1{
				direction = range.min;
			}
			else {
				range.max = (range.min + (range.max - 1)) / 2;
			}
		}
		else if chars[i as usize] == upper_half {
			if range.max - range.min == 1 {
				direction = range.max;
			}
			else {
				range.min = (range.min + (range.max - 1)) / 2 + 1;
			}
		}
	}
	return direction;
}
