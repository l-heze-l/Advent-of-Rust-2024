use std::fs::File; 
use std::io::{self, BufRead};

fn main() -> io::Result<()> {

	// INITIATION //
	let file_path:&str = "data/input.txt";
	let file: File = File::open(file_path)?; 
	let reader: io::BufReader<File> = io::BufReader::new(file);

	let mut data: Vec<(u64, Vec<u64>)> = Vec::new();
	let mut temp_result: u64;

	let mut ans1: u64 = 0;
	let mut ans2: u64 = 0;


	// DATA AQUISITION //
	for line in reader.lines() { 
		let line: String = line?; 
		let parts: Vec<&str> = line.split(':').collect(); 
		let key: u64 = parts[0].trim().parse().unwrap(); 
		let values: Vec<u64> = parts[1]
			.trim()
			.split_whitespace()
			.map(|s| s.parse().unwrap())
			.collect(); 
		data.push((key, values));
	}


	for (result,numbers) in data {
		let num_ops: usize = numbers.len()-1;	// determine the number of operations for each line
	
		// PART 1 //
		for bin in 0..(2_u64.pow(num_ops as u32)) { // generate numbers for possible op combos
			let bin_str: String = format!("{:0num_ops$b}", bin); // cast the numbers as binary strings
			temp_result = numbers[0]; // initiate the first data point
			
			for idx in 0..num_ops { 
				match bin_str.chars().nth(idx).unwrap() { 
					'1' => {
						temp_result += numbers[idx+1];
					}
					'0' => {
						temp_result *= numbers[idx+1];
					}
					_ => (), 
				} 
			}

			if temp_result == result {
				ans1 += result as u64;
				break;
			}
		}

		// PART 2 //
		for ter in 0..(3_u64.pow(num_ops as u32)) { // generate numbers for possible op combos
			let ter_str: String = to_ternary(ter as u32, num_ops); // cast the numbers as ternary strings
			temp_result = numbers[0]; // initiate the first data point

			for idx in 0..num_ops { 
				match ter_str.chars().nth(idx).unwrap() { 
					'2' => {
						let next_num = numbers[idx+1];
						let cat_str: String = format!("{temp_result}{next_num}");
						temp_result = cat_str.parse::<u64>().unwrap();
					}
					'1' => {
						temp_result += numbers[idx+1];
					}
					'0' => {
						temp_result *= numbers[idx+1];
					}
					_ => (), 
				} 
			}

			if temp_result == result {
				ans2 += result as u64;
				println!("{result}");
				break;
			}
		}
	}	


	// DISPLAY //
	println!("Part 1 total: {ans1}");
	println!("Part 2 total: {ans2}");

    Ok(())
}


fn to_ternary(mut number: u32, min_digits: usize) -> String {
	if number == 0 {
			return "0".repeat(min_digits);
	}
	let mut ternary = String::new();
	while number > 0 {
			let remainder = number % 3;
			ternary.push_str(&remainder.to_string());
			number /= 3;
	}
	let mut result = ternary.chars().rev().collect::<String>(); // Reverse the string
	while result.len() < min_digits {
			result.insert(0, '0'); // Prepend '0' to ensure minimum digits
	}
	result
}