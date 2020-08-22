fn main() {
    let string = String::from("apple");
    println! {"{} is {} in pig latin", string, pig(&string)};
}

fn pig(s: &String) -> String {
    let mut pig = String::from(s);
    let first: &str = &pig[0..1]; // this is dangerous
    if vowel(first) {
        pig.push_str("-hay");
    } else {
        pig = format!("{}-{}ay", &pig[1..], first);
    }
    pig.to_string()
}

fn vowel(c: &str) -> bool {
    match c {
        "a" => true,
        "e" => true,
        "i" => true,
        "o" => true,
        "u" => true,
        _ => false,
    }
}
