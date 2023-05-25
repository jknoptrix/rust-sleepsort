use std::{thread, time};

trait SleepSortable {
    fn sleepsort(&self);
}

impl SleepSortable for Vec<u64> {
    fn sleepsort(&self) {
        // create a vector of thread handles by mapping over the numbers in the vector
        let handles: Vec<_> = self
            .into_iter()
            .map(|&n| {
                thread::spawn(move || {
                    // spawn a new thread
                    let duration = time::Duration::from_millis(n); // create a duration equal to the number
                    thread::sleep(duration); // sleep for the duration
                    println!("{}", n); // why?
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

pub fn main() {
    let numbers: Vec<u64> = vec![2, 84, 11, 485, 20, 33];
    numbers.sleepsort();
}
