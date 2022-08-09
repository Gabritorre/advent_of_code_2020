use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input.txt").unwrap();
	let mut lines: Vec<&str> = file.split("\n").collect();
	lines.pop();

	let mut result: i32 = 0;
	let mut password;
	let mut policy;
	let mut letter;
	let mut positions;
	let mut position1: usize;
	let mut position2: usize;
	for i in 0..lines.len() {
		password = lines[i].split(":").last().unwrap();
		password = &password[1..password.len()];
		policy = lines[i].split(":").next().unwrap();
		letter = policy.split(" ").last().unwrap();
		positions = policy.split(" ").next().unwrap();
		position1 = positions.split("-").next().unwrap().parse().unwrap();
		position2 = positions.split("-").last().unwrap().parse().unwrap();
		let letter = letter.chars().next().unwrap();
		if (get_char(&password, &position1) == letter) && (get_char(&password, &position2) != letter) || (get_char(&password, &position1) != letter) && (get_char(&password, &position2) == letter) {
			result += 1;
		}
	}
	println!("{result}");

}

fn get_char(password :&str, position :&usize) -> char {
	return password.chars().nth(position-1).unwrap();
}

