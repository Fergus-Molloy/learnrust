use std::io;

fn main() {
    println!("Pick the term you want");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to Read");

    let num:u64 = num.trim().parse().expect("Failed to convert");

    println!("{}, is the {}th term", fib(num), num);
}

fn fib(n:u64) -> u64 {
    if n == 0 {
        0
    }
    else if n == 1 {
        1
    }
    else {
        fib(n-1) + fib(n-2)
    }
}
