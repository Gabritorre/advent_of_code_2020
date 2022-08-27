use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let file = File::open("input.txt").unwrap();
	let reader = BufReader::new(file);
	let result: i64;
	let mut lines: Vec<i32> = Vec::new();
	for line in reader.lines() {
		lines.push(line.unwrap().parse().unwrap());
	}

	let actual_jolt = 0;
	let max_jolt = lines.iter().max().unwrap();
	let mut children: Vec<i64> = vec![0;(max_jolt + 1) as usize];
	children[*(max_jolt) as usize] = 1;

	result = get_children(actual_jolt, max_jolt, &lines, &mut children);
	println!("{result}");
}

// an example will be more effective than a long explenation
// in example 1 firstly I go from 0 to 19 taking the path that take all the number:
// (0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
// of course 19 have 1 path to reach 22
// 16, 15, 12, 11 as well have 1 path
// 10 have another path which go to 12, but 12 i already know that have 1 path to reach 22, then 10
// will have 2 total path
// 7 and 6 have 2 path as well
// 5 has another path but I know that 7 have 2 path, so 5 will have 4 path
// 4 has two other path and as before I know that 6 have 2 path and 7 has 2 path, so 4 will have 8
// path
// 1 and 0 will have 8 path as well
fn get_children(actual_jolt: i32, max_jolt: &i32, lines: &Vec<i32>, children: &mut Vec<i64>) -> i64 {
	if &actual_jolt == max_jolt {
		return 1;
	}
	let mut children_number: i64 = 0;
	let mut visited = 0;
	for i in 1..=3 {
		if lines.contains(&(actual_jolt + i)) {
			children_number = get_children(actual_jolt + i, max_jolt, lines, children);
			visited = i;
			break;
		}
	}
	if lines.contains(&(actual_jolt + 2)) && visited != 2 {
		children_number += children[(actual_jolt + 2) as usize];
	}
	if lines.contains(&(actual_jolt + 3)) && visited != 3 {
		children_number += children[(actual_jolt + 3) as usize];
	}
	children[actual_jolt as usize] = children_number;
	return children_number;
}

