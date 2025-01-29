
pub fn is_palindrome(x: i32) -> bool {
    let s: String = format!("{x}");
    if s == s.chars().rev().collect::<String>() {
        true
    } else {
        false
    }
}