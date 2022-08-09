use std::fs::read_to_string;
use std::convert::TryInto;

fn main() {
	let file = read_to_string("input.txt").unwrap();
	let mut lines: Vec<&str> = file.split("\n").collect();
	lines.pop();
	let total_columns = lines[0].len().try_into().unwrap();
	let right = [1, 3, 5, 7, 1];
	let mut pattern_result :u64;
	let mut result :u64 = 1;
	let mut column :i32;
	let mut row;
	let mut remaining_columns :i32;

	for i in 0..right.len(){
		pattern_result = 0;
		column = 0;
		row = 0;
		while row < lines.len(){
			if lines[row].chars().nth(column.try_into().unwrap()).unwrap() == '#' {
				pattern_result += 1;
			}
			remaining_columns = get_remaining_columns(column, total_columns);
			if remaining_columns >= right[i] {
				column += right[i];
			}
			else {
				column = right[i] - remaining_columns - 1;
			}
			if i == (right.len() - 1) {
				row += 2;
			}
			else {
				row += 1;
			}
		}
		result *= pattern_result;
	}
	println!("{result}");
}

fn get_remaining_columns(current_column :i32, total_columns :i32) -> i32 {
	return total_columns - (current_column + 1);
}
