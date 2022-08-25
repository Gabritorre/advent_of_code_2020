use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let file = File::open("input.txt").unwrap();
	let reader = BufReader::new(file);
	let result;
	let mut lines: Vec<i64> = Vec::new();
	let mut invalid_number: i64 = 0;
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
		invalid_number = lines[i].clone();
		break;
	}

	min = 0;
	max = 100;
	let mut total = 0;
	let mut numbers: Vec<i64> = Vec::new();
	'outer: for _i in 0..900 {
		for j in min..max {
			total += lines[j];
			numbers.push(lines[j]);
			if invalid_number == total{
				break 'outer;
			}
		}
		min += 1;
		max += 1;
		total = 0;
		numbers.clear();
	}

	result = numbers.iter().min().unwrap() + numbers.iter().max().unwrap();
	println!("{result}");
}

