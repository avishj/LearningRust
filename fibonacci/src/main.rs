use std::io;
fn main() {
    println!("This program prints the N'th Fibonacci number!");

    let mut n: String = String::new();

    println!("Enter integer N:");

    io::stdin().read_line(&mut n).expect("N should be input!");

    let n: u32 = n.trim().parse().expect("N should be a positive number!");

    let mut count: u32 = 2;

    match n {
        0 => println!("N should be greater than 0!"),
        1 => println!("0"),
        2 => println!("1"),
        _ => {
            let mut second_previous: u32 = 0;
            let mut first_previous: u32 = 1;
            let mut current: u32 = first_previous + second_previous;
            while count < n {
                current = first_previous + second_previous;
                second_previous = first_previous;
                first_previous = current;
                count += 1;
            }
            println!("The {n}th Fibonacci Number is: {current}");
        }
    }
}
