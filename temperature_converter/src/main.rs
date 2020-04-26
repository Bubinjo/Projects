use std::io;

fn main() {
    println!("This is temperature converter from Celsius to Fahrenheit");
    println!("Enter temperature in Celsius: ");

    loop {
        let mut temp_input_celsius = String::new();

        io::stdin().read_line(&mut temp_input_celsius).expect("Failed to read line");

        let temp_input_celsius: f32 = match temp_input_celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        let fahrenheit: f32 = temp_input_celsius * 1.8 + 32.0;

        println!("The temperature in Fahrenheit is {}", fahrenheit);
    }
}