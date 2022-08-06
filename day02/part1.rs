use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input.txt").unwrap();
	let mut lines: Vec<&str> = file.split("\n").collect();
	lines.pop();

	let mut result: i32 = 0;
	let mut password;
	let mut policy;
	let mut letter;
	let mut range;
	let mut min: usize;
	let mut max: usize;
	let mut occurrences;
	for i in 0..lines.len() {
		password = lines[i].split(":").last().unwrap();
		password = &password[1..password.len()];
		policy = lines[i].split(":").next().unwrap();
		letter = policy.split(" ").last().unwrap();
		range = policy.split(" ").next().unwrap();
		min = range.split("-").next().unwrap().parse().unwrap();
		max = range.split("-").last().unwrap().parse().unwrap();
		occurrences = password.matches(letter).count();
		if occurrences >= min && occurrences <= max {
			result += 1;
		}
	}
	println!("{result}");

}

