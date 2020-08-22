use std::thread;
use std::time::Duration;

fn main() {
    let simulate_user_input = 10;
    let simulate_random = 7;

    generate_workout(simulate_user_input, simulate_random);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...",);
        thread::sleep(Duration::from_micros(2000));
        num
    });

    if intensity < 25 {
        println!("do {} puchups", expensive_result.value(intensity));
        println!("do {} situps", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break for today",);
        } else {
            println!("run for {} minutes", expensive_result.value(intensity));
        }
    }
}
