// Convert Fahrenheit to Celsius
use std::io;

fn calculate(deg: f64) -> f64 { (deg - 32.0) * (5.0 / 9.0) }

pub fn main() {
    let mut temperature = String::new();
    println!("Enter a temperature in Fahrenheit:");

    io::stdin()
        .read_line(&mut temperature)
        .expect("Error reading input.");
    
    let temperature: f64 = temperature
        .trim()
        .parse()
        .expect("Temperature must be a number!");
    
    let result = calculate(temperature);
    println!("{:.2}°F => {:.2}°C", temperature, result);
}
