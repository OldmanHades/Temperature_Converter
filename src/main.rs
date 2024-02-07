use std::io;

fn main() {
    loop {
        println!("Choose a conversion:");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice. Please enter 1, 2, or 3.");
                continue;
            }
        };

        match choice {
            1 => convert_fahrenheit_to_celsius(),
            2 => convert_celsius_to_fahrenheit(),
            3 => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn convert_fahrenheit_to_celsius() {
    println!("Please input a temperature in Fahrenheit:");

    let mut fahrenheit_str = String::new();
    io::stdin()
        .read_line(&mut fahrenheit_str)
        .expect("Failed to read line");

    let fahrenheit: f64 = match fahrenheit_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{}F is equal to {}C", fahrenheit, celsius);
}

fn convert_celsius_to_fahrenheit() {
    println!("Please input a temperature in Celsius:");

    let mut celsius_str = String::new();
    io::stdin()
        .read_line(&mut celsius_str)
        .expect("Failed to read line");

    let celsius: f64 = match celsius_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{}C is equal to {}F", celsius, fahrenheit);
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}
