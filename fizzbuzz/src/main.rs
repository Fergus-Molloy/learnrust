use std::io;

fn main() {
    println!("Pick which number to go up to");
    let mut last = String::new();

    io::stdin().read_line(&mut last).expect("failed to read");

    let last: u32 = last.trim().parse().expect("Failed to convert");

    for num in 1..(last + 1) {
        if (num % 3 == 0) && (num % 5 == 0) {
            println!("fizzbuzz");
        } else if num % 5 == 0 {
            println!("buzz");
        } else if num % 3 == 0 {
            println!("fizz");
        } else {
            println!("{}", num);
        }
    }
}
