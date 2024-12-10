use std::fs::File; 
use std::io::{self, BufRead};
fn main() -> io::Result<()> {

	// INITIATION //
	let file_path:&str = "data/input.txt";
	let file: File = File::open(file_path)?; 
	let reader: io::BufReader<File> = io::BufReader::new(file);

	let mut rules: Vec<(i32,i32)> = Vec::new();
	let mut page_sets: Vec<Vec<i32>> = Vec::new();
	let mut passes: i32;
	let mut checked: bool;

	let mut ans1: i32 = 0;
	let mut ans2: i32 = 0;


	// DATA AQUISITION //
	for line in reader.lines() { 
		let line: String = line?; 

		// Get the first sets of numbers (the rules)
		if line.contains('|') { 
			let parts: Vec<&str> = line.split('|').collect(); 
			let num1: i32 = parts[0].parse().unwrap(); 
			let num2: i32 = parts[1].parse().unwrap(); 
			rules.push((num1, num2)); 
		} 
		
		// Get the second sets of numbers (the pages)
		else if line.contains(',') { 
			let parts: Vec<&str> = line.split(',').collect(); 
			let nums: Vec<i32> = parts.iter().map(|&s| s.trim().parse().unwrap()).collect(); 
			page_sets.push(nums); 
		} 
	}


	// PART 1 //
	for pages in &page_sets {
		passes = 0;
		for (one,two) in &rules {
			if pages.contains(one) && pages.contains(two) {
				if pages.iter().position(|&x| x == *one) < pages.iter().position(|&x| x == *two) {
					//println!("W - {one}|{two} - {:?}", pages);
					passes += 1;
				} 
			} else {
				passes += 1;
			}
		}
		if passes >= rules.len() as i32 {
			ans1 += pages[pages.len()/2];
		}


		// PART 2 //
		else {
			let mut page_fix: Vec<i32> = pages.clone();
			checked = false;
			while !checked {
				checked = true;
				for (one,two) in &rules {
					if page_fix.contains(one) && page_fix.contains(two) {
						if page_fix.iter().position(|&x| x == *one) > page_fix.iter().position(|&x| x == *two) {
							//println!("{:?} - {one}|{two}", page_fix);
							let idx1: usize = page_fix.iter().position(|&x| x == *one).unwrap();
							let idx2: usize = page_fix.iter().position(|&x| x == *two).unwrap();
							page_fix[idx2] = *one;
							page_fix[idx1] = *two;
							checked = false;
						} 
					} 
				}
			}
			ans2 += page_fix[page_fix.len()/2];
			//println!("{ans2} - {:?}", page_fix);
		}
	}


	// DISPLAY //
	println!("Part 1 total: {ans1}");
	println!("Part 2 total: {ans2}");

    Ok(())
}


