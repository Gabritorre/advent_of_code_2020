use std::fs::File;
use std::io::{BufRead, BufReader};
use std::convert::TryInto;

fn main() {
	let file = File::open("input.txt").unwrap();
	let reader = BufReader::new(file);
	let result;
	let mut lines: Vec<String> = Vec::new();
	for line in reader.lines() {
		lines.push(line.unwrap());
	}
	let directions = ['E', 'N', 'W', 'S'];
	let mut y_value = 0;
	let mut x_value = 0;
	let mut facing = 'E';

	for line in lines.iter_mut() {
		let action = line.remove(0);
		let value: i32 = line.parse().unwrap();
		do_actions(action, &value, &directions, &mut y_value, &mut x_value, &mut facing);
	}
	result = x_value.abs() + y_value.abs();
	println!("{result}");
}

fn do_actions(action: char, value: &i32, directions: &[char;4], y_value: &mut i32, x_value: &mut i32, facing: &mut char) {
	match action {
		'E' => *x_value += value,
		'N' => *y_value += value,
		'W' => *x_value -= value,
		'S' => *y_value -= value,
		'F' => do_actions(*facing, value, directions, y_value, x_value, facing),
		'R' => {
			let turns = value/90;
			let mut index: i32 = directions.iter().position(|x| x == facing).unwrap().try_into().unwrap();
			if index - turns < 0 {
				index = (directions.len() - (turns - index) as usize).try_into().unwrap();
			}
			else{
				index -= turns;
			}
			*facing = directions[index as usize];
		},
		'L' => {
			let turns = value/90;
			let mut index: i32 = directions.iter().position(|x| x == facing).unwrap().try_into().unwrap();
			if index + turns > 3 {
				let length: i32 = directions.len().try_into().unwrap();
				index = length - (turns + index);
				index = index.abs();
			}
			else{
				index += turns;
			}
			*facing = directions[index as usize];
		},
		_ => todo!(),
	}
}
