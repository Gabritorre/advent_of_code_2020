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
	let mut ship_y_value = 0;
	let mut ship_x_value = 0;
	let mut waypoint_y_value = 1;
	let mut waypoint_x_value = 10;

	for line in lines.iter_mut() {
		let action = line.remove(0);
		let value: i32 = line.parse().unwrap();
		do_actions(action, &value, &mut ship_y_value, &mut ship_x_value, &mut waypoint_y_value, &mut waypoint_x_value);
	}
	result = ship_x_value.abs() + ship_y_value.abs();
	println!("{result}");
}

fn do_actions(action: char, value: &i32, ship_y_value: &mut i32, ship_x_value: &mut i32, waypoint_y_value: &mut i32, waypoint_x_value: &mut i32) {
	match action {
		'E' => *waypoint_x_value += value,
		'N' => *waypoint_y_value += value,
		'W' => *waypoint_x_value -= value,
		'S' => *waypoint_y_value -= value,
		'F' => {
			let x_move: i32 = value * *waypoint_x_value;
			let y_move: i32 = value * *waypoint_y_value;
			*ship_y_value += y_move;
			*ship_x_value += x_move;
		},
		'R' | 'L' => {
			let turns = value/90;
			if (turns == 1 && action == 'R') || (turns == 3 && action == 'L') {
				let temp = *waypoint_x_value * -1;
				*waypoint_x_value = *waypoint_y_value;
				*waypoint_y_value = temp;
			}
			else if turns == 2 {
				*waypoint_y_value *= -1;
				*waypoint_x_value *= -1;
			}
			else if (turns == 3 && action == 'R') || (turns == 1 && action == 'L') {
				let temp = *waypoint_y_value * -1;
				*waypoint_y_value = *waypoint_x_value;
				*waypoint_x_value = temp;
			}
		},
		_ => todo!(),
	}
}
