use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Initially set previous_group number total to the highest u16 integer value.
    let mut previous_measurement_group_total = u16::MAX;
    // Initialize our total increases counter to zero.
    let mut total_increases: u16 = 0;
    // Initialize our triad total to zero
    let mut current_measurement_group_total: u16 = 0;
    // Keep track of our index into our grouping.
    let mut measurement_number_group_index = 0;
    // Maximum number of measurements per group.
    const MAXIMUM_MEASUREMENT_PER_GROUP: usize = 3;
    // Measurement grouping
    let mut measurement_group_exists: bool = false;
    // Measurement Window
    let mut array: [u16; MAXIMUM_MEASUREMENT_PER_GROUP] = [0; MAXIMUM_MEASUREMENT_PER_GROUP];
    
    // Open file "input.txt" for reading.
    let file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(file);

    // File reading code reference: https://riptutorial.com/rust/example/4275/read-a-file-line-by-line
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {

        let line = line.unwrap();
        let current_number: u16 = line.trim().parse().expect("Please enter a number");
        // Increase our number group index by one.
        measurement_number_group_index +=1;

        match measurement_number_group_index.cmp(&MAXIMUM_MEASUREMENT_PER_GROUP) {
            Ordering::Less => {
                // Decrease our index by one for assigning numbers to the array.
                array[measurement_number_group_index-1] = current_number;
            }
            Ordering::Greater => {
                // Do nothing
            }
            Ordering::Equal => {
                // All group numbers received, so update the measurement total.
                current_measurement_group_total = current_number + array[0] + array[1];
                // Shift the array values to the left in preparation for the next value.
                array[0] = array[1];
                array[1] = current_number;                
                measurement_group_exists = true;
            }
        }

        // Since we have all 3 items for the window, do the necessary processing and housekeeping.
        if measurement_group_exists {
            // Capture our measurement increase, if the previous is less than the current.
            if previous_measurement_group_total < current_measurement_group_total {
                total_increases += 1;
            }
            // Reset our measurement boolean to false.
            measurement_group_exists = false;
            // Set our current group total to our previous group total.
            previous_measurement_group_total = current_measurement_group_total;
            // Decrease our array index by one as it will be incremented during the next loop.
            measurement_number_group_index -= 1;
            // Reset our current measurement group total.
            current_measurement_group_total = 0;
  
        }
    }

    println!("The total number of depth measurement increases: {}", total_increases);

}
