use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let file = File::open("input.txt").unwrap();
	let reader = BufReader::new(file);
	let mut result = 0;
	let mut lines: Vec<String> = Vec::new();
	for line in reader.lines() {
		lines.push(line.unwrap());
	}
	let mut visited_lines: Vec<i32> = vec![0; lines.len()];
	result = get_acc_value(&lines, &mut visited_lines);
	println!("{:?}", visited_lines);
	println!("{result}");
}

fn get_acc_value (lines: &Vec<String>, visited_lines: &mut Vec<i32>) -> i32 {
	let mut accumulator = 0;
	let mut index = 0;
	loop{
		println!("{}", index);
		if visited_lines[index] == 1 {
			println!("loop!");
			break;
		}

		visited_lines[index] = 1;
		if lines[index].contains("acc") {
			accumulator += lines[index].split(" ").last().unwrap().parse::<i32>().unwrap();
			index += 1;
		}
		else if lines[index].contains("jmp") {
			if lines[index].contains("+") {
				index += lines[index].split("+").last().unwrap().parse::<usize>().unwrap();
			}
			else {
				index -= lines[index].split("-").last().unwrap().parse::<usize>().unwrap();
			}
		}
		else {
			index += 1;
		}
	}
	return accumulator;
}
