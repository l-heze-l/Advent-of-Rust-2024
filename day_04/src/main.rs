use std::char;
use std::fs::File; 
use std::io::{self, BufRead}; 
mod matrix_utils;

fn main() -> io::Result<()> {

	// INITIATION //
	let file_path:&str = "data/input.txt";
	let file: File = File::open(file_path)?; 
	let reader: io::BufReader<File> = io::BufReader::new(file);

	let mut matrix: Vec<Vec<char>> = Vec::new();
	let mut dirs: Vec<(isize,isize)>;
	let x_dirs: [(isize, isize); 4] = [ 
		(-1, -1), (-1, 1), (1, -1), (1, 1) 
	];

	let mut ans1: i32 = 0;
	let mut ans2: i32 = 0;


	// DATA AQUISITION //
	for line in reader.lines() { 
			let line = line?; // Convert the line into a vector of characters and add it to the matrix 
			matrix.push(line.chars().collect());
    }


	// PART 1 //
	for y_u in 0..matrix.len() {
		let y = y_u as isize;

		for x_u in 0..matrix[0].len() {
			let x = x_u as isize;

			if let Some(&value) = matrix_utils::get_value_at(&matrix, y, x) {
				if value != 'X' {
					continue;
				}
			}
			dirs = search_surrounding(&matrix, 'M', (y,x));

			for (dy, dx) in dirs {
				if let Some(&value) = matrix_utils::get_value_at(&matrix, y+2*dy, x+2*dx) {
					if value != 'A' {
						continue;
					}
					if let Some(&value) = matrix_utils::get_value_at(&matrix, y+3*dy, x+3*dx) {
						if value != 'S' {
							continue;
						}
						ans1 += 1;
					} 
				} 
			}
		}
	}


	// PART 2 //
	for y_u in 0..matrix.len() {
		let y: isize = y_u as isize;

		for x_u in 0..matrix[0].len() {
			let x: isize = x_u as isize;

			if let Some(&value) = matrix_utils::get_value_at(&matrix, y, x) {
				if value != 'A' {
					continue;
				}
				let mut m = 2;
				let mut s = 2;

				for (dy,dx) in x_dirs {
					if let Some(&value) = matrix_utils::get_value_at(&matrix, y+dy, x+dx) {
						match value {
							'M' => m -= 1,
							'S' => s -= 1,
							_ => continue,
						}
						if let Some(&o_val) = matrix_utils::get_value_at(&matrix, y-dy, x-dx) {
							if value == o_val {
								continue;
							}
						}
					}
					if m==0 && s==0 {
						ans2 += 1;
					}
				}
			}
		}
	}


	// DISPLAY //
	println!("Part 1 total: {ans1}");
	println!("Part 2 total: {ans2}");

    Ok(())
}


fn search_surrounding(matrix: &Vec<Vec<char>>, letter: char, idx: (isize, isize)) -> Vec<(isize,isize)> { 
	let mut dirs: Vec<(isize,isize)> = Vec::new();

	let directions: [(isize, isize); 8] = [ 
		(-1, -1), (-1, 0), (-1, 1), (0, -1), 
		(0, 1), (1, -1), (1, 0), (1, 1), 
	];

	for (dy, dx) in directions { 
		if let Some(&value) = matrix_utils::get_value_at(matrix, idx.0 + dy, idx.1 + dx) { 
			if value == letter { 
				dirs.push((dy, dx)); 
			} 
		} 
	}

	return dirs;
}


