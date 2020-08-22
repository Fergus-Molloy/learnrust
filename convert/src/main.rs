use std::io;

fn main() {
    loop {
        println!("Type in a number to convert to Fahrenheit");

        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read");

        let num:i32 = match num.trim().parse() {
            Ok(number) => number,
            Err(_)  => continue,
        };

        println!("{}°C is {} Fahrenheit", num, convert(num));
        break;
    }
}

fn convert(x:i32) -> i32 {
    (x * (9/5)) + 32
}
