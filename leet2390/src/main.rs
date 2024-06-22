fn remove_stars(s: String) -> String {
    let mut stack: Vec<char> = Vec::new();
    for ch in s.chars() {
        if ch =='*' {
            stack.pop();
        } else {
            stack.push(ch);
        }
   }

   stack.iter().collect()
}

fn main() {
    let s = String::from("leet**cod*e");
    remove_stars(s);
}