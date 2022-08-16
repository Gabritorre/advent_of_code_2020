use std::fs::File;
use std::io::{BufRead, BufReader};

static mut POSITIVE_BAGS: Vec<String> = Vec::new();
static mut MAIN_BAGS: Vec<String> = Vec::new();

fn main() {
	let file = File::open("input.txt").unwrap();
	let reader = BufReader::new(file);
	let mut result = 0;
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
		for j in 0..10 {
			let mut number = " ".to_string();
			number.push_str(&j.to_string());
			lines[i] = lines[i].replace(&number, "");
		}
		lines[i].pop();
//		println!("{:?}", lines[i]);
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
			MAIN_BAGS.push(bag_name.clone());
//			println!("pushato {} in indice {}", bag_name, MAIN_BAGS.len()-1);
		}
	}

	for i in 0..lines.len() {
//		println!("{:?}", lines[i]);
		result += find_shiny_gold(i, &lines, i);
	}
	println!("{result}");
}

fn find_shiny_gold(index: usize, lines: &Vec<String>, main_line_index: usize) -> i32 {
	if lines[index].contains(&"no other".to_string()) {
//		println!("no other! a {}", lines[index]);
		return 0;
	}
	if lines[index].contains("shiny gold") && lines[index][..10] != "shiny gold".to_string() {
		unsafe {
			POSITIVE_BAGS.push(MAIN_BAGS[index].clone());
		}
//		println!("trovato shiny gold a {}", lines[index]);
		unsafe {
//			println!("POSITIVE_BAGS: {:?}", POSITIVE_BAGS);
		}
		return 1;
	}
	for (i, bag) in lines[index].split(" , ").enumerate() {
		if i == 0 {
			continue;
		}
//		println!("prova |{}|", bag);
		unsafe {
			if POSITIVE_BAGS.iter().position(|x| x == bag) != None {
//				println!("trovato in positive");
				return 1;
			}
		}
		let mut new_bag_index: usize = index;
		unsafe {
			if MAIN_BAGS.iter().position(|x| x == bag) != None {
				new_bag_index = MAIN_BAGS.iter().position(|x| x == bag).unwrap();
				if new_bag_index == main_line_index {
					continue;
				}
			}
			else {
//				println!("dio cane");
				continue;
			}
		}
//		println!("prova linea {} ci dovrebbe essere {}", &new_bag_index, bag);
		if find_shiny_gold(new_bag_index, lines, main_line_index) != 0 {
			unsafe {
				POSITIVE_BAGS.push(MAIN_BAGS[index].clone());
				return 1;
			}
		}
	}
//	println!("hello!");
	return 0;
}
