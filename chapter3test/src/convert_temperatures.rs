use std::io;

pub(crate) fn main() {
    println!("Choose conversion type:");
    println!("1: Fahrenheit to Celsius");
    println!("2: Celsius to Fahrenheit");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = choice.trim().parse().expect("Invalid choice");

    println!("Enter the temperature you want to convert");
    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature).expect("Failed to read input");
    let temperature: f64 = temperature.trim().parse().expect("Invalid temperature");

    match choice {
        1 => {
            let celsius: f64 = (temperature - 32.0) * 5.0 / 9.0;
            println!("{temperature}°F is {celsius:.2}°C");
        }
        2=>{
            let fahrenheit: f64 = temperature*9.0/5.0 + 32.0;
            println!("{temperature}°C is {fahrenheit:.2}°F");
        }
        _=>{
            println!("Invalid choice");
        }
    }
}