use std::collections::HashMap;

fn main() {
    println!("test",);
}
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_arg() {
        let mut cache = Cacher::new(|x| x);
        assert_eq!(cache.value(1), 1);
    }

    #[test]
    fn two_arg() {
        let mut cache = Cacher::new(|x| x);
        let result = cache.value(1);
        assert_eq!(cache.value(2), 2);
    }
}
