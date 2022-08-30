use std::fs::File;
use std::io::{BufRead, BufReader};
use std::convert::TryInto;


//to avoid index out of bound I have put my input in a frame of '.' like that:
//..............
//.my input.txt.
//..............
fn main() {
	let file = File::open("input.txt").unwrap();
	let reader = BufReader::new(file);
	let mut result = 0;
	let mut lines: Vec<String> = Vec::new();
	for line in reader.lines() {
		lines.push(line.unwrap());
	}
	let width = lines[0].len();
	let height = lines.len();
	let mut matrix = vec![vec!['.';width + 2]; height + 2];
	for i in 1..height + 1 {
		for j in 1..width + 1 {
			matrix[i][j] = lines[i - 1].chars().nth(j - 1).unwrap();
		}
	}
	let mut edited_matrix = matrix.clone();

	loop {
		//loop each character in the input, the frame of '.' are excluded
		for row in 1..height + 1 {
			for column in 1..width + 1 {
				if matrix[row][column] == 'L' {
					if check_visibility(&matrix, row, column) == 0 {
						edited_matrix[row][column] = '#';
						result += 1;
					}
				}
				else if matrix[row][column] == '#' {
					if check_visibility(&matrix, row, column) >= 5 {
						edited_matrix[row][column] = 'L';
						result -= 1;
					}
				}
			}
		}
		if edited_matrix == matrix {
			break;
		}
		matrix = edited_matrix.clone();
	};
	println!("{result}");
}
// direction array help me to know if a direction is already visited (true = visited, false = not
// visited, the indexes rappresent the directions clockwise: 1 = up, 2 = up-right, 3 = right etc...
// counter take count of the occurences of '#'
// I have a for loop and in each cycle I watch in each direction going deeper every loop
fn check_visibility(matrix: &Vec<Vec<char>>, row: usize, column: usize) -> i32 {
	let mut counter = 0;
	let mut directions = [false;8];
	let right_limit:i32 = matrix[0].len().try_into().unwrap();
	let bottom_limit:i32 = matrix.len().try_into().unwrap();
	let test_row: i32 = row.try_into().unwrap();
	let test_column: i32 = column.try_into().unwrap();

	for i in 1..matrix[0].len() {
		let index: i32 = i.clone().try_into().unwrap();
		if test_row - index > 0 && directions[0] == false { // up
			if matrix[(test_row - index) as usize][column] != '.' {
				directions[0] = true;
				if matrix[(test_row - index) as usize][column] == '#' {
					counter += 1;
				}
			}
		}
		if (test_row - index > 0) && (test_column + index < right_limit) && directions[1] == false { // up-right
			if matrix[(test_row - index) as usize][(test_column + index) as usize] != '.' {
				directions[1] = true;
				if matrix[(test_row - index) as usize][(test_column + index) as usize] == '#' {
					counter += 1;
				}
			}
		}
		if test_column + index < right_limit && directions[2] == false { // right
			if matrix[row][(test_column + index) as usize] != '.' {
				directions[2] = true;
				if matrix[row][(test_column + index) as usize] == '#' {
					counter += 1;
				}
			}
		}
		if (test_row + index < bottom_limit) && (test_column + index < right_limit) && directions[3] == false { // down-right
			if matrix[(test_row + index) as usize][(test_column + index) as usize] != '.' {
				directions[3] = true;
				if matrix[(test_row + index) as usize][(test_column + index) as usize] == '#' {
					counter += 1;
				}
			}
		}
		if (test_row + index < bottom_limit) && directions[4] == false { // down
			if matrix[(test_row + index) as usize][column] != '.' {
				directions[4] = true;
				if matrix[(test_row + index) as usize][column] == '#' {
					counter += 1;
				}
			}
		}
		if (test_row + index < bottom_limit) && (test_column - index > 0) && directions[5] == false { // down-left
			if matrix[(test_row + index) as usize][(test_column - index) as usize] != '.' {
				directions[5] = true;
				if matrix[(test_row + index) as usize][(test_column - index) as usize] == '#' {
					counter += 1;
				}
			}
		}
		if test_column - index > 0 && directions[6] == false { // left
			if matrix[row][(test_column - index) as usize] != '.' {
				directions[6] = true;
				if matrix[row][(test_column - index) as usize] == '#' {
					counter += 1;
				}
			}
		}
		if (test_row - index > 0) && (test_column - index > 0) && directions[7] == false { // up-left
			if matrix[(test_row - index) as usize][(test_column - index) as usize] != '.' {
				directions[7] = true;
				if matrix[(test_row - index) as usize][(test_column - index) as usize] == '#' {
					counter += 1;
				}
			}
		}
	}
	return counter;
}
