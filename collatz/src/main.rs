fn collatz_steps_count(num: i64) -> Option<i64> {
    match num {
        0 => None,
        1 => Some(0),
        num if num % 2 ==0 => collatz_steps_count(num / 2).map(|steps| steps + 1),
        num => collatz_steps_count(num * 3 + 1).map(|steps| steps + 1),
    }
}