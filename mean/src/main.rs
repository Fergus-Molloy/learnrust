use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 4, 3, 5, 5];
    println!("The mean of {:?} is: {}", v, mean(&v));
    println!("The median of {:?} is: {}", v, median(&v));
    println!("The mode of {:?} is: {}", v, mode(&v));
    println!("The mode of {:?} is: {}", v, mode0(&v));
}

fn mean(v: &Vec<i32>) -> i32 {
    let mut total = 0;
    for x in v {
        total += x;
    }
    total / (v.len() as i32)
}

fn median(v: &Vec<i32>) -> i32 {
    let half = v.len() / 2;
    let median = match v.get(half) {
        Some(val) => val,
        None => panic!("No value in vector"),
    };
    *median
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for x in v {
        let count = map.entry(x).or_insert(0 as i32);
        *count += 1;
    }

    let mut mode: (i32, i32) = (0, 0);
    for (key, value) in &map {
        if value > &mode.1 {
            mode = (**key, *value);
        }
    }
    mode.0
}

fn mode0(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for x in v {
        let count = map.entry(x).or_insert(0 as i32);
        *count += 1;
    }

    let mut mode: i32 = 0;
    for (key, value) in &map {
        if value > &mode {
            if let Some(_) = map.get(key) {
                mode = **key;
            }
        }
    }
    mode
}
