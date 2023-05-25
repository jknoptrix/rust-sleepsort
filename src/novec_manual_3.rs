/* TRAITS */
use std::{env, thread, time};

trait SleepSortable {
    fn sleepsort(&self);
}

impl SleepSortable for Vec<u64> {
    fn sleepsort(&self) {
        let handles: Vec<_> = self
            .into_iter()
            .map(|&n| {
                thread::spawn(move || {
                    let duration = time::Duration::from_millis(n);
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

// check first novec
pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1..].iter().map(|x| x.parse()).collect::<Result<Vec<u64>, _>>() {
            Ok(numbers) => numbers.sleepsort(),
            Err(e) => eprintln!("Error parsing arguments: {}", e),
        }
    } else {
        eprintln!("numbers?");
    }
}

