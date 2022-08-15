use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let file = File::open("input.txt").unwrap();
	let reader = BufReader::new(file);
	let mut first_group_line = "".to_string();
	let mut result = 0;
	let mut new_group = true;
	for line in reader.lines() {
		let line = line.unwrap();
		if new_group {
			first_group_line = line.clone();
			new_group = false;
		}

		if line.is_empty() {
			result += first_group_line.len();
			new_group = true;
		}
		else{
			let copy_line = first_group_line.clone();
			for letter in copy_line.chars(){
				if ! line.contains(letter){
					first_group_line = first_group_line.replace(&letter.to_string(), "");
				}
			}
		}
	}
	result += first_group_line.len(); // last remaining group
	println!("{result}");
}
