use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let mut alphabet = HashMap::new();
    alphabet.insert('I', 1);
    alphabet.insert('V', 5);
    alphabet.insert('X', 10);
    alphabet.insert('L', 50);
    alphabet.insert('C', 100);
    alphabet.insert('D', 500);
    alphabet.insert('M', 1000);

    let mut total = 0;
    let mut previous_value = 0;

    for ch in s.chars().rev() {
        let value = alphabet[&ch];

        if value < previous_value {
            total -= value;
        } else {
            total += value;
        }

        previous_value = value;
    }

    total
}

fn main() {
    println!("{}", roman_to_int("LVIII".to_string()));
    println!("{}", roman_to_int("III".to_string()));
    println!("{}", roman_to_int("MCMXCIV".to_string()));
}