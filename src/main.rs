use std::io;

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}
fn main() {
    //Q-1 out the fibonacci sequence
    println!("Enter a number : ");
    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("Enter a number");

    let number: u32 = number.trim().parse().expect("enter a proper number");

    println!("The fibonacci of {} is {}", number, fibonacci(number));
}
