/* NO TRAITS */
use std::{env, thread, time};

pub fn sleepsort(numbers: &[u64]) {
    let mut handles = vec![];
    for &num in numbers {
        let handle = thread::spawn(move || {
            thread::sleep(time::Duration::from_millis(num));
            println!("{}", num);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

// check first novec
pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let numbers: Vec<u64> = args[1..].iter().map(|x| x.parse().unwrap()).collect();
        sleepsort(&numbers);
    } else {
        eprintln!("числа укажи");
    }
}
