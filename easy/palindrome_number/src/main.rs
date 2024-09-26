pub fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }

    let mut result = 0;
    let mut tmp = x;

    while tmp > 0 {
        result = result * 10 + tmp % 10;
        tmp /= 10;
    }

    if result == x {
        return true;
    }

    false
}

fn main() {
    println!("{}", is_palindrome(121));
    println!("{}", is_palindrome(-121));
}
