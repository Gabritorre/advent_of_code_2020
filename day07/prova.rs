// salvare tute le righe in un array
// scorrere una volta tutte le righe contare quante volte si trova il "shiny gold" e salvare il nome del
// beg che lo contiene in un altro array
// riscorrere tutte le righe controllando per ogni riga se sono presenti le beg salvate nell'array
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let file = File::open("input.txt").unwrap();
	let reader = BufReader::new(file);
	let mut result = 0;
	let mut lines: Vec<String> = Vec::new();
	let mut valid_bags: Vec<String> = Vec::new();
	for line in reader.lines() {
		lines.push(line.unwrap());
	}

	for line in lines.iter() {
		if line.contains("shiny gold") && line[..10] != "shiny gold".to_string() {
			let mut bag_name = "".to_string();
			for (i, word) in line.split_whitespace().enumerate() {
				if i < 2 {
					bag_name.push_str(" "); // putting a whitespace at the beginning of the string allow me to exclude the name of the beg at the beginning of each line the input file
					bag_name.push_str(word);
				}
			}
			valid_bags.push(bag_name);
			result += 1;
		}
	}
	println!("{:?}", valid_bags);
	for line in lines.iter() {
		for beg in valid_bags.iter() {
			if line.contains(beg) {
				result += 1;
				println!("{} |CONTIENE| {}", line, beg);
				break;
			}
		}
	}
	println!("{result}");
}
