use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input.txt").unwrap();
	let mut lines: Vec<&str> = file.split("\n").collect();
	lines.pop();

	let mut first_value: i32;
	let mut second_value: i32;
	let mut third_value: i32;
	let mut result: i32 = 0;
	for i in 0..lines.len() {
		first_value = lines[i].parse().unwrap();
		for j in 0..lines.len() {
			second_value = lines[j].parse().unwrap();
			for h in 0..lines.len(){
				third_value = lines[h].parse().unwrap();
				if (first_value + second_value + third_value) == 2020 {
					result = first_value * second_value * third_value;
				}
			}
		}
	}
	println!("{result}");
}

