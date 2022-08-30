use std::fs::File;
use std::io::{BufRead, BufReader};

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
	//fill the matrix
	for i in 1..height + 1 {
		for j in 1..width + 1 {
			matrix[i][j] = lines[i - 1].chars().nth(j - 1).unwrap();
		}
	}
	let mut edited_matrix = matrix.clone();
	loop {
		for row in 1..height + 1 {
			for column in 1..width + 1 {
				if matrix[row][column] == 'L' {
					if check_adjacent(&matrix, row, column) == 0 {
						edited_matrix[row][column] = '#';
						result += 1;
					}
				}
				else if matrix[row][column] == '#' {
					if check_adjacent(&matrix, row, column) >= 4 {
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

fn check_adjacent(matrix: &Vec<Vec<char>>, row: usize, column: usize) -> i32 {
	let mut count_occupied = 0;
	for i in row-1..=row+1 {
		for j in column-1..=column+1 {
			if i == row && j == column {
				continue;
			}
			if matrix[i][j] == '#' {
				count_occupied += 1;
			}
		}
	}
	return count_occupied;
}
