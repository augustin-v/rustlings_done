
fn decode_string(s: String) -> String {
    let mut stack_num = Vec::new();
    let mut stack_str = Vec::new();
    let mut current_num = 0;
    let mut current_str = String::new();
    
    for c in s.chars() {
        if c.is_digit(10) {
            current_num = current_num * 10 + c.to_digit(10).unwrap();
        } else if c == '[' {
            stack_num.push(current_num);
            stack_str.push(current_str.clone());
            current_num = 0;
            current_str = String::new();
        } else if c == ']' {
            let num = stack_num.pop().unwrap();
            let mut temp = stack_str.pop().unwrap();
            for _ in 0..num {
                temp.push_str(&current_str);
            }
            current_str = temp;
        } else {
            current_str.push(c);
        }
    }
    
    current_str
}