use std::fs::read_to_string;
use std::convert::TryInto;

const RIGHT :i32 = 3;

fn main() {
	let file = read_to_string("input.txt").unwrap();
	let mut lines: Vec<&str> = file.split("\n").collect();
	lines.pop();
	let total_columns = lines[0].len().try_into().unwrap();
	let mut result :i32 = 0;
	let mut column :i32 = 0;
	let mut remaining_columns :i32;
	for row in 0..lines.len() {
		if lines[row].chars().nth(column.try_into().unwrap()).unwrap() == '#' {
			result += 1;
		}
		remaining_columns = get_remaining_columns(column, total_columns);
		if remaining_columns >= RIGHT {
			column += RIGHT;
		}
		// if there is no space to move in the right direction, we have to restart the index in the same line
		// eg: we are in the second-last column (we can't move right of 3 spaces) so we move 1 in
		// the right direction and the 2 remaining moves restart from the beginning of the line.
		// the '-1' is because index start from 0
		else {
			column = RIGHT - remaining_columns - 1;
		}
	}
	println!("{result}");
}

fn get_remaining_columns(current_column :i32, total_columns :i32) -> i32 {
	return total_columns - (current_column + 1);
}
