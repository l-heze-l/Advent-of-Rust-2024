use std::fs;
use regex::Regex;

fn main() {

	// INITIATION //
	let file_path = "data/input.txt";
	let mut ans1: i32 = 0;
	let mut ans2: i32 = 0;
	let mut enable: bool = true;
	let mut x: i32;
	let mut y: i32;

	// DATA AQUISITION //
	let content: String = fs::read_to_string(file_path) 
		.expect("Something went wrong reading the file");


	// PART 1 //
	let re1: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

	for caps in re1.captures_iter(&content) { 
		x = caps.get(1).unwrap().as_str().parse().unwrap(); 
		y = caps.get(2).unwrap().as_str().parse().unwrap(); 
		ans1 = ans1 + x * y;
	}


	// PART 2 //
	let re2: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

	for caps in re2.captures_iter(&content) { 
		if let Some(mul_caps) = caps.get(1) { 
			if enable {
				x = mul_caps.as_str().parse().unwrap(); 
				y = caps.get(2).unwrap().as_str().parse().unwrap(); 
				ans2 = ans2 + x * y; 
			}
		} 
		else if caps.get(0).unwrap().as_str() == "do()" { 
			enable = true;
		} 
		else if caps.get(0).unwrap().as_str() == "don't()" { 
			enable = false;
		}
	}


	// DISPLAY //
	println!("Part 1 total: {ans1}");
	println!("Part 2 total: {ans2}");
}
