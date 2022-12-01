use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
        // Initially set max calories to zero.
        let mut max_calories: u32 = 0;
        // Initialize our running_total.
        let mut running_total: u32 = 0;
        
        // Open file "input.txt" for reading.
        let file = File::open("input.txt").expect("File not found");
        let reader = BufReader::new(file);
    
        // File reading code reference: https://riptutorial.com/rust/example/4275/read-a-file-line-by-line
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for (_index, line) in reader.lines().enumerate() {
    
            let line = line.unwrap(); // Ignore errors.

            match line.trim().parse::<u32>() {
                Err(_n) => {
                    if running_total > max_calories {
                        max_calories = running_total;
                    }
                    running_total = 0;
                },
                Ok(_n) => {
                    let current_number: u32 = line.trim().parse().expect("Please enter a number");
                    running_total = running_total + current_number;
                }
            }

    
        }
        // Grab the last line item for comparison
        if running_total > max_calories {
            max_calories = running_total;
        }
    
        println!("Max calories: {}", max_calories);
}
