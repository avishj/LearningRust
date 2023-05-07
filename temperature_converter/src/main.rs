use std::io;

fn main() {
    println!("Enter the input temperature scale: F / C");

    let mut scale = String::new();
    io::stdin()
        .read_line(&mut scale)
        .expect("Scale should be input!");

    let scale: char = scale
        .trim()
        .parse()
        .expect("Scale should be a single letter!");

    println!(
        "Enter the temperature in {}:",
        if scale == 'C' { "Celcius" } else { "Farenheit" }
    );

    let mut input_temperature = String::new();

    io::stdin()
        .read_line(&mut input_temperature)
        .expect("Temperature should be input!");

    let input_temperature: i32 = input_temperature
        .trim()
        .parse()
        .expect("Temperature should be a number!");

    match scale {
        'C' => {
            println!(
                "The temperature in fahrenheit is: {}",
                (input_temperature * 9 / 5) + 32
            );
        }
        'F' => {
            println!(
                "The temperature in celsius is: {}",
                (input_temperature - 32) * (5 / 9)
            );
        }
        _ => {
            println!("Scale should be either C / F. ")
        }
    }
}
