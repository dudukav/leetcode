pub fn str_str(haystack: String, needle: String) -> i32 {
    haystack.find(&needle).map(|i| i as i32).unwrap_or(-1)
}

fn main() {
    let haystack = String::from("sadbutsad");
    let needle = String::from("sad");
}
