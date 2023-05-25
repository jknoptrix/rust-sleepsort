use std::{thread, time};

// define a trait named SleepSortable with one required method named sleepsort
trait SleepSortable {
    fn sleepsort(&self);
}

// implement the "SleepSortable" trait for vectors of unsigned 64-bit integers
impl SleepSortable for Vec<u64> {
    fn sleepsort(&self) {
        let handles: Vec<_> = self
            .into_iter()
            .map(|&n| {
                thread::spawn(move || {
                    // im tired to write the same comments every fucking file
                    let duration = time::Duration::from_millis(n); // create a duration equal to the number
                    thread::sleep(duration);
                    println!("{}", n);
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

pub fn main(numbers: Vec<u64>) {
    numbers.sleepsort();
}
