use std::fs::File; 
use std::io::{self, BufRead}; 
use std::path::Path;

fn main() -> io::Result<()> {

	// INITIATION //
	let path: &Path = Path::new("data/input.txt");
	let file: File = File::open(&path)?; 
	let reader: io::BufReader<File> = io::BufReader::new(file);

	let mut ans1: i32 = 0;
	let mut ans2: i32 = 0;
	let mut validity: usize;


	// DATA AQUISITION //
	for line in reader.lines() { 
		let line: String = line?; 
		let numbers: Vec<i32> = line 
			.split_whitespace() 
			.map(|s| s.parse().expect("parse error")) 
			.collect(); 

		// PART 1 //
		validity = 1;
		match numbers[0]-numbers[1] {
			1 | 2 | 3 => {			// decreasing     
				for i in 0..(numbers.len()-1) {
					if numbers[i]-numbers[i+1]==1 || numbers[i]-numbers[i+1]==2 || numbers[i]-numbers[i+1]==3 {
						validity = validity + 1;
					}
				}
				if validity >= numbers.len() {
					ans1 = ans1 + 1;
				}
			},
			-1 | -2 | -3 => {		// increasing
				for i in 0..(numbers.len()-1) {
					if numbers[i]-numbers[i+1]==-1 || numbers[i]-numbers[i+1]==-2 || numbers[i]-numbers[i+1]==-3 {
						validity = validity + 1;
					}
				}
				if validity >= numbers.len() {
					ans1 = ans1 + 1;
				}
			},
			_ => {/* do nothing otherwise */}
		}


		// PART 2 //
		let num_arr: Vec<i32> = numbers.to_vec();
		let mut num_damp: Vec<i32>;
		for i in 0..(num_arr.len()) {
			num_damp = num_arr.clone();
			num_damp.remove(i);
			validity = 1;

			match num_damp[0]-num_damp[1] {
				1 | 2 | 3 => {			// decreasing     
					for j in 0..(num_damp.len()-1) {
						if num_damp[j]-num_damp[j+1]==1 || num_damp[j]-num_damp[j+1]==2 || num_damp[j]-num_damp[j+1]==3 {
							validity = validity + 1;
						}
					}
					if validity >= num_damp.len() {
						ans2 = ans2 + 1;
						break;
					}
				},
				-1 | -2 | -3 => {		// increasing
					for j in 0..(num_damp.len()-1) {
						if num_damp[j]-num_damp[j+1]==-1 || num_damp[j]-num_damp[j+1]==-2 || num_damp[j]-num_damp[j+1]==-3 {
							validity = validity + 1;
						}
					}
					if validity >= num_damp.len() {
						ans2 = ans2 + 1;
						break;
					}
				},
				_ => {/* do nothing otherwise */}
			}
		}
	}


	// DISPLAY //
	println!("Part 1 Total: {ans1}");
	println!("Part 2 Total: {ans2}");

	Ok(())
}
