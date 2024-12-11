use std::fs::File; 
use std::io::{self, BufRead};
mod matrix_utils;

fn main() -> io::Result<()> {

	// INITIATION //
	let file_path:&str = "data/test.txt";
	let file: File = File::open(file_path)?; 
	let reader: io::BufReader<File> = io::BufReader::new(file);

	let mut map: Vec<Vec<char>> = Vec::new();
	let mut start_idx: (isize,isize) = (0,0);
	let mut current_idx: (isize,isize) = (0,0);
	let mut back_idx: (isize,isize);

	let dirs: [(isize,isize,char);4] = [(-1,0,'^'),(0,1,'>'),(1,0,'v'),(0,-1,'<')];
	let mut dir_idx: usize = 0;

	let mut done: bool = false;

	let mut ans1: i32 = 0;
	let mut ans2: i32 = 0;


	// DATA AQUISITION //
	for line in reader.lines() { 
		let line: String = line?; 
		map.push(line.chars().collect());
	}
	for y in 0..map.len() {
		if map[y].contains(&'^') {
			start_idx = (y as isize, map[y].iter().position(|&x| x == '^').unwrap() as isize);
			current_idx = start_idx;
		}
	}


	// PART 1 //
	let mut x_map: Vec<Vec<char>> = map.clone();
	matrix_utils::isize_insert(&mut x_map, current_idx.0, current_idx.1, 'X');
	ans1 += 1;
	loop {
		let next: Option<&char> = matrix_utils::isize_index(&x_map, current_idx.0+dirs[dir_idx].0, current_idx.1+dirs[dir_idx].1);
		match next {
			Some(_) => {
				match next.unwrap() {
					'.' => {
						current_idx = (current_idx.0+dirs[dir_idx].0,current_idx.1+dirs[dir_idx].1);
						matrix_utils::isize_insert(&mut x_map, current_idx.0, current_idx.1, 'X');
						ans1 += 1;
					}
					'#' => {
						dir_idx = (dir_idx + 1) % 4;
					}
					'X' => {
						current_idx = (current_idx.0+dirs[dir_idx].0,current_idx.1+dirs[dir_idx].1);
					}
					_ => break
				};
			}
			None => break
		};
	}
	//for row in x_map { for char in row {print!("{}", char);} println!() }


	// PART 2 //
	current_idx = start_idx;
	dir_idx = 0;
	let mut v_map: Vec<Vec<char>> = map.clone();
	let mut paths: Vec<Vec<Vec<char>>> = vec![vec![Vec::new(); v_map[0].len()]; v_map.len()];
	matrix_utils::isize_push(&mut paths,current_idx.0, current_idx.1, dirs[dir_idx].2);

	while !done {
		// Make a back-trail
		back_idx = current_idx;
		loop {
			let next: Option<&char> = matrix_utils::isize_index(&v_map, back_idx.0-dirs[dir_idx].0, back_idx.1-dirs[dir_idx].1);
			match next {
				Some(_) => {
					match next.unwrap() {
						'.'|'^'|'>'|'v'|'<' => {
							back_idx = (back_idx.0-dirs[dir_idx].0,back_idx.1-dirs[dir_idx].1);
							matrix_utils::isize_insert(&mut v_map, back_idx.0, back_idx.1, dirs[dir_idx].2);
							matrix_utils::isize_push(&mut paths,back_idx.0, back_idx.1, dirs[dir_idx].2);
						}
						_ => break
					};
				}
				None => {
					break;
				}
			};
		}	

		// Continue the Path
		loop {
			let next: Option<&char> = matrix_utils::isize_index(&v_map, current_idx.0+dirs[dir_idx].0, current_idx.1+dirs[dir_idx].1);
			match next {
				Some(_) => {
					match next.unwrap() {
						'.'|'O' => {
							current_idx = (current_idx.0+dirs[dir_idx].0,current_idx.1+dirs[dir_idx].1);
							matrix_utils::isize_insert(&mut v_map, current_idx.0, current_idx.1, dirs[dir_idx].2);
							matrix_utils::isize_push(&mut paths,current_idx.0, current_idx.1, dirs[dir_idx].2);
						}
						'^'|'>'|'v'|'<' => {
							current_idx = (current_idx.0+dirs[dir_idx].0,current_idx.1+dirs[dir_idx].1);
							matrix_utils::isize_push(&mut paths,current_idx.0, current_idx.1, dirs[dir_idx].2);
							if paths[current_idx.0 as usize][current_idx.1 as usize].contains(&dirs[(dir_idx+1)%4].2)  {
								ans2 += 1;
								matrix_utils::isize_insert(&mut v_map, current_idx.0, current_idx.1, 'O');
							}
							else {
								matrix_utils::isize_insert(&mut v_map, current_idx.0, current_idx.1, dirs[dir_idx].2);
							}
						}
						'#' => {
							dir_idx = (dir_idx + 1) % 4;
							matrix_utils::isize_insert(&mut v_map, current_idx.0, current_idx.1, dirs[dir_idx].2);
							matrix_utils::isize_push(&mut paths,current_idx.0, current_idx.1, dirs[dir_idx].2);
							break;
						}
						_ => {
							done = true;
							break;
						}
					};
				}
				None => {
					done = true;
					break;
				}
			};
		}	
		
	}
	//for row in v_map { for char in row {print!("{}", char);} println!() }
	//for row in &paths {for vec in row {print!("{:?}", vec);} println!();}

	// DISPLAY //
	println!("Part 1 total: {ans1}");
	println!("Part 2 total: {ans2}");

    Ok(())
}


