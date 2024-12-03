use std::fs::File; 
use std::io::{self, BufRead}; 
use std::path::Path;

fn main() -> io::Result<()> {

    // initiation //
    let path: &Path = Path::new("data/input.txt");
    let file: File = File::open(&path)?; 
    let reader: io::BufReader<File> = io::BufReader::new(file);
    let mut column1: Vec<i32> = Vec::new(); 
    let mut column2: Vec<i32> = Vec::new();
    let mut ans1: i32 = 0;
    let mut ans2: i32 = 0;
    let mut diff: i32;

    // data aquisition //
    for line in reader.lines() { 
        let line: String = line?; 
        let numbers: Vec<i32> = line 
            .split_whitespace() 
            .map(|s| s.parse().expect("parse error")) 
            .collect(); 
        
        // Assuming each line has exactly two numbers 
        if numbers.len() == 2 { 
            column1.push(numbers[0]); 
            column2.push(numbers[1]); 
        } 
    }

    // calculation //
    column1.sort();     
    column2.sort();

    for idx in 0..column1.len() {
        diff = column1[idx] - column2[idx];
        ans1 = ans1 + diff.abs();
    }

    // display //
    println!("Part 1 Total: {ans1}");
    println!("Part 2 Total: {ans2}");

    Ok(())
}
