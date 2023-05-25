use std::{thread, time};

// define a macro named "sleepsort"
// u ask why - i say "fuck you"
// :)
macro_rules! sleepsort {
    // as soon the macro takes one argument: an expression that evaluates to a collection of numbers
    ($numbers:expr) => {{
        let mut handles = vec![]; // create an empty vector to store thread handles
        for num in $numbers {
            // for each number in the collection
            let handle = thread::spawn(move || {
                // spawning a ----zombie---- a new thread
                thread::sleep(time::Duration::from_millis(num)); // sleep for a duration equal to the number
                println!("{}", num);
            });
            handles.push(handle); // store the thread handle in the vector
        }
        for handle in handles {
            // for each thread handle in the vector
            handle.join().unwrap(); // wait for the thread to finish
        }
    }};
}

// define a public function named main that takes a vector of unsigned 64-bit integers as an argument
pub fn main(numbers: Vec<u64>) {
    sleepsort!(numbers); // call the sleepsort macro with the numbers as an argument
}
