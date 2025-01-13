use ordinal::Ordinal;
use std::collections::HashMap;
use std::io;

pub fn fibonacci_stuff() {
    println!("Please enter the number n value for nth element of the fibonacci sequence");

    let mut memo = HashMap::new();

    let nth = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid read");
        match input.trim().parse::<u128>() {
            Ok(num) => {
                if num > 186 {
                    println!(
                        "Please enter a number less than 187...my poor u128 can't take any more..."
                    );
                    continue;
                } else {
                    break num;
                }
            }
            Err(_) => {
                println!("Please enter a non-negative number");
                continue;
            }
        };
    };

    println!(
        "The {} element of the Fibonacci sequence is: {}",
        Ordinal(nth),
        fibo_calc(nth, &mut memo)
    );
}

fn fibo_calc(n: u128, memo: &mut HashMap<u128, u128>) -> u128 {
    if let Some(&result) = memo.get(&n) {
        return result;
    }

    let result = if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibo_calc(n - 1, memo) + fibo_calc(n - 2, memo)
    };

    memo.insert(n, result);
    result
}
