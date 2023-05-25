use std::{thread, time};

// MCCROOOOO DEFINNEEEEEEEEEEEEEEEEEEEEEEEEEEEEE
macro_rules! sleepsort {
    // the macro should take a variable number of arguments separated by commas
    ($($num:expr),*) => {{
        let mut handles = vec![]; // create an empty vector to store thread handles
        $(
            // "foreach" argument passed to the macro
            let handle = thread::spawn(move || {
                thread::sleep(time::Duration::from_millis($num));
                println!("{}", $num);
            });
            handles.push(handle);
        )*
        for handle in handles {
            handle.join().unwrap();
        }
    }};
}

pub fn main() {
    sleepsort!(20, 10, 40, 600); // call the sleepsort macro with 4 or more arguments
}
