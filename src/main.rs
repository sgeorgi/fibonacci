use std::io;

fn fibonacci(x: u128) -> u128 {
    match x {
        0 => 0,
        1 | 2 => 1,
        _ => fibonacci(x - 1) + fibonacci(x - 2)
    }
}


fn main() {
    println!("Welcome to the Fibonacci Calculator");
    println!("\nPlease specify the n in Fn!");

    let mut character: String = String::new();

    io::stdin()
        .read_line(&mut character)
        .expect("Failed to read line");

    let number: u128 = match character.trim().parse() {
        Ok(num) => { num }
        Err(_) => {
            println!("No number given, bailing out!");
            panic!()
        }
    };

    println!("Calculating the {}nth Fibonacci number ... ", number);
    println!("{}", fibonacci(number))
}
