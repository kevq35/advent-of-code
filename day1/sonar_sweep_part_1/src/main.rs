use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Initially set previous_number to the highest u16 integer value.
    let mut previous_number = u16::MAX;
    // Initialize our total increases counter to zero.
    let mut total_increases: u16 = 0;
    
    // Open file "input.txt" for reading.
    let file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(file);

    // File reading code reference: https://riptutorial.com/rust/example/4275/read-a-file-line-by-line
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {

        let line = line.unwrap();
        let current_number: u16 = line.trim().parse().expect("Please enter a number");

        match current_number.cmp(&previous_number) {
            Ordering::Less => {
            }
            Ordering::Greater => {
                total_increases += 1;
            }
            Ordering::Equal => {
            }
        }

        previous_number = current_number;

    }

    println!("The total number of depth measurement increases: {}", total_increases);

}
