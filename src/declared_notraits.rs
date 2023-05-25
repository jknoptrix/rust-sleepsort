use std::{thread, time};

// unsigned 64-bit integers as an argument???????????????????????????????????????????? giga
fn sleepsort(numbers: &[u64]) {
    let handles: Vec<_> = numbers
        .into_iter()
        .map(|&n| {
            thread::spawn(move || {
                let duration = time::Duration::from_millis(n);
                thread::sleep(duration);
                println!("{}", n)
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn main() {
    let numbers: Vec<u64> = vec![2, 84, 11, 485, 20, 33];
    sleepsort(&numbers); // call the sleepsort function with a reference to the numbers as an argument
}
