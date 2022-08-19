use std::fs::File;
use std::io::{BufRead, BufReader};

static mut RESULT: i32 = 0;
static mut MAIN_BAGS: Vec<String> = Vec::new();
static mut STACK: Vec<i32> = Vec::new();

// for the main explenation read the part 1
// the only differences are that the numbers are not removed
fn main() {
	let file = File::open("input.txt").unwrap();
	let reader = BufReader::new(file);
	let mut shiny_gold_index = 0;
	let mut lines: Vec<String> = Vec::new();
	for line in reader.lines() {
		lines.push(line.unwrap());
	}

	for i in 0..lines.len() {
		lines[i] = lines[i].replace("bags.", "");
		lines[i] = lines[i].replace("bags", "");
		lines[i] = lines[i].replace("bag.", "");
		lines[i] = lines[i].replace("bag", "");
		lines[i] = lines[i].replace(" contain ", ", ");
		lines[i].pop();
	}
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
			if bag_name == "shiny gold" {
				shiny_gold_index = MAIN_BAGS.len();
			}
			MAIN_BAGS.push(bag_name.clone());
		}
	}

	find_shiny_gold(shiny_gold_index, &lines, shiny_gold_index);
	unsafe {
		println!("{}", RESULT);
	}
}

// if the line contains "no other" stop the recursion (this is the only way to stop the recursion)
// loop through each bag name of the line (excluding the first one)
// get the number of the current bag
// remove the number to get only the name of the bag
// get the line number of the bag
// save in a global stack the bag number
// once the recursion stop i pop the last inserted number from the stack and multiply that number
// for each remaining number in the stack
// after moltiply all the number in the stack a sum the resutl in the RESULT global variable
// i think that this program work thanks to a lot of lucky (lol), the stack doesn't return wrong
// value because of the '1' doesn't change the result of a moltiplication, if the first bag_number
// were 2 (or a number different than 1) the result will be completely broken
fn find_shiny_gold(index: usize, lines: &Vec<String>, shiny_gold_index: usize) {
	let mut bag_number: i32;
	if lines[index].contains(&"no other".to_string()) {
		return ();
	}
	for (i, bag_numbered) in lines[index].split(" , ").enumerate() {
		if i == 0 {
			continue;
		}

		bag_number = bag_numbered.split(" ").next().unwrap().parse().unwrap();
		let mut bag = "".to_string();
		for j in 0..10 {
			let mut del_number = "".to_string();
			del_number.push_str(&j.to_string());
			del_number.push_str(&" ".to_string());
			bag = bag_numbered.replace(&del_number, "");
			if bag != bag_numbered {
				break;
			}
		}

		let new_bag_index: usize;
		unsafe {
			if MAIN_BAGS.iter().position(|x| x == &bag) != None {
				new_bag_index = MAIN_BAGS.iter().position(|x| x == &bag).unwrap();
				if new_bag_index == shiny_gold_index {
					continue;
				}
			}
			else {
				continue;
			}
		}
		unsafe {
			STACK.push(bag_number);
		}
		find_shiny_gold(new_bag_index, lines, shiny_gold_index);
		unsafe {
			let mut leaf = STACK.pop();
			for i in 0..STACK.len() {
				leaf = Some(leaf.unwrap() * STACK[i]);
			}
			RESULT += leaf.unwrap();
		}
	}
}
