use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let file = File::open("input.txt").unwrap();
	let reader = BufReader::new(file);
	let mut present_letters = "".to_string();
	let mut result = 0;

	for line in reader.lines() {
		let line = line.unwrap();
		if line.is_empty() {
			present_letters = "".to_string();
		}
		else{
			for letter in line.chars(){
				if ! present_letters.contains(letter){
					present_letters.push_str(&letter.to_string());
					result += 1;
				}
			}
		}
	}
	println!("{result}");
}
