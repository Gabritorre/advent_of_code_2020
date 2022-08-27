use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let file = File::open("input.txt").unwrap();
	let reader = BufReader::new(file);
	let result;
	let mut lines: Vec<i32> = Vec::new();
	for line in reader.lines() {
		lines.push(line.unwrap().parse().unwrap());
	}
	let mut difference_one = 0;
	let mut difference_three = 1;
	let mut actual_jolt = 0;
	let max_jolt = lines.iter().max().unwrap();
	while &actual_jolt != max_jolt {
		if lines.contains(&(actual_jolt + 1)) {
			difference_one += 1;
			actual_jolt = actual_jolt + 1;
		}
		else if lines.contains(&(actual_jolt + 3)) {
			difference_three += 1;
			actual_jolt = actual_jolt + 3;
		}
	}
	result = difference_one * difference_three;
	println!("{result}");
}

