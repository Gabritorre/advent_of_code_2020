use std::fs::File;
use std::io::{BufRead, BufReader};

static mut POSITIVE_BAGS: Vec<String> = Vec::new(); // vector that contains all the bag that can contain at least a shiny gold bag
static mut MAIN_BAGS: Vec<String> = Vec::new(); // vector that contains all the bag at the beginning of each line in the input file (all the unique bag)

fn main() {
	let file = File::open("input.txt").unwrap();
	let reader = BufReader::new(file);
	let mut result = 0;
	let mut lines: Vec<String> = Vec::new();
	for line in reader.lines() {
		lines.push(line.unwrap());
	}
//	removing useless stuff from the lines transforming them in "bag_name , bag_name , bag_name , etc..."
	for i in 0..lines.len() {
		lines[i] = lines[i].replace("bags.", "");
		lines[i] = lines[i].replace("bags", "");
		lines[i] = lines[i].replace("bag.", "");
		lines[i] = lines[i].replace("bag", "");
		lines[i] = lines[i].replace(" contain ", ", ");
		for j in 0..10 {
			let mut number = " ".to_string();
			number.push_str(&j.to_string());
			lines[i] = lines[i].replace(&number, "");
		}
		lines[i].pop();
	}
//	fill the MAIN_BAGS vector
	for line in lines.iter() {
		let mut bag_name = "".to_string();
		for (i, word) in line.split_whitespace().enumerate() {
			if i < 2 {
				if i == 1 {
					bag_name.push_str(" ");
				}
				bag_name.push_str(word);
			}
		}
		unsafe {
			MAIN_BAGS.push(bag_name.clone());
		}
	}
//	loop line by line to find if a line can bag a shiny gold bag
	for i in 0..lines.len() {
		result += find_shiny_gold(i, &lines, i);
	}
	println!("{result}");
}
// recursive method to find if a line can bag a shiny gold bag: 1 if the original bag can contains it, otherwise 0
// if the line contains "no other" stop the recursion with 0;
// if the line contains shiny gold, but that "shiny gold" had not to be the first bag (look the example in the website), then stop the recursion with 1. Also save the bag that contains directly the shiny gold bag in POSITIVE_BAGS
// then split the line getting all the bag of the line and loop through it
// ignore the first bag
// if the bag is present in the POSITIVE_BAGS means that bag is already been checked and it can contains
// a shiny gold bag
// if none of the previous conditions were true, we get the number of the line where the name of the bag is equal to the name of the first bag in a line of the input (MAIN_BAGS)
// if the number of the line we get is equal to main_line_index (number of original line where we start [the upper for loop]) then we ignore that bag to avoid infinity loop
// in conclusion if the result of the recursive method is 1 we save the name of the valid bags (that contains shiny gold bag) in the POSITIVE_BAGS
fn find_shiny_gold(index: usize, lines: &Vec<String>, main_line_index: usize) -> i32 {
	if lines[index].contains(&"no other".to_string()) {
		return 0;
	}
	if lines[index].contains("shiny gold") && lines[index][..10] != "shiny gold".to_string() {
		unsafe {
			POSITIVE_BAGS.push(MAIN_BAGS[index].clone());
		}
		return 1;
	}
	for (i, bag) in lines[index].split(" , ").enumerate() {
		if i == 0 {
			continue;
		}
		unsafe {
			if POSITIVE_BAGS.iter().position(|x| x == bag) != None {
				return 1;
			}
		}

		let new_bag_index: usize;
		unsafe {
			if MAIN_BAGS.iter().position(|x| x == bag) != None {
				new_bag_index = MAIN_BAGS.iter().position(|x| x == bag).unwrap();
				if new_bag_index == main_line_index {
					continue;
				}
			}
			else {
				continue;
			}
		}

		if find_shiny_gold(new_bag_index, lines, main_line_index) != 0 {
			unsafe {
				POSITIVE_BAGS.push(MAIN_BAGS[index].clone());
				return 1;
			}
		}
	}
	return 0; // if find_shiny_gold return 0
}
