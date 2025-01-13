use std::io;

pub fn degree_calc() {
    println!("Please enter 1 for C to F, or 2 for F to C");

    let option = loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<u32>() {
                Ok(num) if (1..=2).contains(&num) => {
                    break num;
                }
                _ => {
                    println!("Please enter a valid option between 1 or 2.");
                }
            },
            Err(_) => {
                println!("Failed to read line.");
            }
        }
    };

    if option == 1 {
        println!("Please enter the number of degrees in celsius");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input: f32 = input.trim().parse().expect("Invalid value");

        let output = input * (9.0 / 5.0) + 32.0;

        println!("{} degrees fahrenheit", output);
    }

    if option == 2 {
        println!("Please enter the number of degrees in fahrenheit");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input: f32 = input.trim().parse().expect("Invalid value");

        let output = (input - 32.0) / 1.8;

        println!("{} degrees celsius", output);
    }
}
