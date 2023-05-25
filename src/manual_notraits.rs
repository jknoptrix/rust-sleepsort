use std::{thread, time};

// define a function named sleepsort that takes a slice of unsigned 64-bit integers as an argument cuz rust
fn sleepsort(numbers: &[u64]) {
    let mut handles = vec![]; // creating an empty vector to store thread handles
    for &num in numbers {
        let handle = thread::spawn(move || {
            // spawning a new thread
            thread::sleep(time::Duration::from_millis(num)); // sleep for a duration equal to the number
            println!("{}", num); // print the number
        });
        handles.push(handle); // store the thread handle in the vector
    }
    for handle in handles {
        handle.join().unwrap(); // wait for the thread to finish
    }
}

// define a public function named main that takes a vector of unsigned 64-bit integers as an argument
pub fn main(numbers: Vec<u64>) {
    sleepsort(&numbers); // call the sleepsort function with a reference to the numbers as an argument
}
