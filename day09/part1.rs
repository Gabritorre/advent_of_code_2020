use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let file = File::open("input.txt").unwrap();
	let reader = BufReader::new(file);
	let mut result = 0;
	let mut lines: Vec<i64> = Vec::new();
	for line in reader.lines() {
		lines.push(line.unwrap().parse().unwrap());
	}
	let mut min = 0;
	let mut max = 25;
	'outer: for i in 25..lines.len() {
		for j in min..max-1 {
			for h in j..max {
				if lines[i] == lines[j] + lines[h] {
					min += 1;
					max += 1;
					continue 'outer;
				}
			}
		}
		result = lines[i].clone();
		break;
	}
	println!("{result}");
}

