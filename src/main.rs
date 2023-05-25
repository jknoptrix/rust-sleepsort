use std::io

mod declared_macro;
mod declared_notraits;
mod declared_traits;
mod manual_macro;
mod manual_notraits;
mod manual_traits;

fn main() {
    println!("Choose a sleepsort method:");
    println!("1. Custom input - Macros");
    println!("2. Custom input - Without traits");
    println!("3. Custom input - With traits");
    println!("4. Declared array - Macros");
    println!("5. Declared array - Without traits");
    println!("6. Declared array - With traits");

    // get the users choice
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // read a line from standard input
    let choice: u32 = input.trim().parse().unwrap(); // parse the input as an unsigned 32-bit integer

    // match the user's choice to the corresponding function
    match choice {
        1 => {
            let numbers = get_numbers_from_user(); // get numbers from the user
            manual_macro::main(numbers); // calling the main function from manual_macro module with the numbers as an argument
        }
        2 => {
            let numbers = get_numbers_from_user(); // same but notraits
            manual_notraits::main(numbers); // same
        }
        3 => {
            let numbers = get_numbers_from_user(); // bro i said same
            manual_traits::main(numbers); // SAME
        }
        4 => declared_macro::main(), // call the main function from declared_macro module
        5 => declared_notraits::main(),
        6 => declared_traits::main(),
        _ => println!("Invalid choice"), // why should i even comment this man
    }
}

// function to get numbers from the user
fn get_numbers_from_user() -> Vec<u64> {
    println!("Enter numbers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // read a line from standard input
    input
        .split_whitespace() // split the input into words separated by whitespace
        .map(|x| x.parse().unwrap()) // parsing each word as an unsigned 64-bit integer
        .collect() // collect the parsed integers into a vector and return it
}
