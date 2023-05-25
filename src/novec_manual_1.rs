
/* MACRO */
use std::{env, thread, time};

// it does actually the same bullshit as declared_macro.rs does so dont worry
macro_rules! sleepsort {
    ($numbers:expr) => {{
        let mut handles = vec![];
        for num in $numbers {
            let handle = thread::spawn(move || {
                thread::sleep(time::Duration::from_millis(num));
                println!("{}", num);
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }};
}
// BUT it have the one different thing in main function:

/* the main difference between this code and the code from manual_*.rs is that this 
code takes its input from the command line arguments while the code from manual_*.rs 
takes its input from an argument passed to the `main` function.
in the code below the user can provide a list of numbers to sort by passing them as command line arguments (cargo run <numbers>) 
i would use this code for main.rs but its impossible cuz it requires the code modification for be a module */
pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let numbers: Vec<u64> = args[1..].iter().map(|x| x.parse().unwrap()).collect();
        sleepsort!(numbers);
    } else {
        eprintln!("enter the numbers");
    }
}