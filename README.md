# Sleepsort

This project demonstrates different implementations of the sleepsort algorithm in Rust for the newbies.

## main.rs

The `main.rs` file contains the main function that runs when the program is executed. It presents the user with a menu of options to choose from and then calls the appropriate function based on the user's choice.

## Modules

The project contains several modules that implement different versions of the sleepsort algorithm:

- `manual_macro.rs`: module that uses a macro to implement sleepsort.
- `manual_notraits.rs`: module that implements sleepsort using a function without using traits.
- `manual_traits.rs`: module that implements sleepsort using a trait and its implementation for vectors of unsigned 64-bit integers.
- `declared_macro.rs`: module that uses a macro to implement sleepsort on a fixed set of numbers defined in the code.
- `declared_notraits.rs`: module that implements sleepsort on a fixed set of numbers defined in the code using a function without using traits.
- `declared_traits.rs`: module that implements sleepsort on a fixed set of numbers defined in the code using a trait and its implementation for vectors of unsigned 64-bit integers.

In `declared` modules you can declare your own number in array:
```rust
pub fn main() {
    let numbers: Vec<u64> = vec![2, 84, 11, 485, 20, 33];
    numbers.sleepsort();
}
```
In manual modules you should type them while program executes. Also you can you separetely declared modules `novec`. They contains a code which can be executed without `main.rs` code:
```rust
pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let numbers: Vec<u64> = args[1..].iter().map(|x| x.parse().unwrap()).collect();
        sleepsort!(numbers);
    } else {
        eprintln!("enter the numbers");
    }
}
```
## Sleepsort

Sleepsort is an unconventional sorting algorithm that takes advantage of the time it takes for threads to sleep. For each number in the collection to be sorted, a new thread is spawned that sleeps for a duration equal to the number. When the thread wakes up, it prints the number. Since threads that sleep for shorter durations will wake up first, the numbers are printed in ascending order.
Well, it's actually ineffective, but fun.

