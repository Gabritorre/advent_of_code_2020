use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let file = File::open("input.txt").unwrap();
	let reader = BufReader::new(file);
	let result;
	let mut lines: Vec<String> = Vec::new();
	for line in reader.lines() {
		lines.push(line.unwrap());
	}
	let mut visited_lines: Vec<i32> = vec![0; lines.len()]; // fill the vector with zeros
	result = find_corruption(&mut lines, &mut visited_lines, 0);
	println!("{result}");
}

// recursive method, base case: if the call encounters a line already visited then return 0
// mark new line as visited and if the line contains jmp then calculate the value of the next index
// otherwise (in case of "acc" or "nop") just make the next index as the actual index + 1
// now call the method until we reach the base case
// and then if we get 0 means that we haven't reach the last line of the input
// so we change one at time the line's operation (jmp in nop and vice versa)
// after we change one line we try to reach the last line in the input: if not, then undo the edit
// made to that line otherwise return the result

// in a few words change the second-last -> try; change the third-last -> try; change the
// fourth-last -> try; and so on until we avoid the infinite loop
fn find_corruption (lines: &mut Vec<String>, visited_lines: &mut Vec<i32>, index: usize) -> i32 {
	let next_index: usize;
	if visited_lines[index] == 1 {
		return 0;
	}

	visited_lines[index] = 1;
	if lines[index].contains("jmp") {
		if lines[index].contains("+") {
			next_index = index + lines[index].split("+").last().unwrap().parse::<usize>().unwrap();
		}
		else {
			next_index = index - lines[index].split("-").last().unwrap().parse::<usize>().unwrap();
		}
	}
	else {
		next_index = index + 1;
	}
	let result = find_corruption(lines, visited_lines, next_index);
	if result == 0 {
		if lines[index].contains("jmp") {
			lines[index] = lines[index].replace("jmp", "nop").clone();
			let accumulator = try_path(lines, visited_lines);
			if accumulator == 0 {
				lines[index] = lines[index].replace("nop", "jmp").clone();
				return 0;
			}
			return accumulator;
		}
		if lines[index].contains("nop") {
			lines[index] = lines[index].replace("nop", "jmp").clone();
			let accumulator = try_path(lines, visited_lines);
			if accumulator == 0 {
				lines[index] = lines[index].replace("jmp", "nop").clone();
				return 0;
			}
			return accumulator;
		}
		else {
			return 0;
		}
	}
	return result;
}

// method that try to reach a result without running into a infinite loop
// if infinite loop return 0
// otherwise reutrn the value of accumulator (the rusult)
fn try_path (lines: &Vec<String>, visited_lines: &mut Vec<i32>) -> i32 {
	let mut accumulator = 0;
	let mut index = 0;

	for i in 0..visited_lines.len() {
		visited_lines[i] = 0;
	}

	while index != lines.len()-1 {
		if visited_lines[index] == 1 {
			return 0;
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
