use std::collections::HashMap;

fn int_to_roman(num: i32) -> String {
    let mut map = HashMap::new();
    map.insert("M", 1000);
    map.insert("CM", 900);
    map.insert("D", 500);
    map.insert("CD", 400);
    map.insert("C", 100);
    map.insert("XC", 90);
    map.insert("L", 50);
    map.insert("XL", 40);
    map.insert("X", 10);
    map.insert("IX", 9);
    map.insert("V", 5);
    map.insert("IV", 4);
    map.insert("I", 1);


    let mut num = num;
    let mut res = String::new();
    let mut values: Vec<_> = map.iter().collect();
    values.sort_by(|a,b| a.cmp(b));

    for &(roman, &value) in values.iter().rev() {
        while num >= value {
            num -= value;
            res.push_str(roman);
        }
    }
    println!("{}", res);
    res
}

fn main () {
    let num = 123;
    int_to_roman(num);
}

#[cfg(test)]
mod test {
    #[test]
    fn basic() {

    }
}