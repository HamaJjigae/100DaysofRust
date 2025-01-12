mod degree_calc;
mod fibonacci;
mod lyrics;

use degree_calc::degree_calc;
use fibonacci::fibonacci_stuff;
use lyrics::lyrics;

use std::io;

fn main() {
    println!("Please select a program:\n\t1 Fahrenheit and Celsius Conversion\n\t2 Generate the nth Fibonacci Number\n\t3 Print the lyrics to The Twelve Days of Christmas");

    let mut input = String::new();

    let option = loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<u32>() {
                Ok(num) if (1..=3).contains(&num) => {
                    break num;
                }
                _ => {
                    println!("Please enter a valid option between 1 and 3.");
                }
            },
            Err(_) => {
                println!("Failed to read line.");
            }
        }
    };

    println!("You selected {}", option);

    match option {
        1 => degree_calc(),
        2 => fibonacci_stuff(),
        3 => lyrics(),
        _ => println!("Invalid option"),
    }
}
