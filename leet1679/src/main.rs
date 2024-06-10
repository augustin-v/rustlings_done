    use std::collections::HashMap;
    
    fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts = HashMap::new();
        let mut res = 0;
    
        for &num in &nums {
            let complement = k - num;
            if let Some(&count) = counts.get(&complement) {
                if count > 0 {
                    res += 1;
                    *counts.get_mut(&complement).unwrap() -= 1;
                } else {
                    *counts.entry(num).or_insert(0) += 1;
                }
            } else {
                *counts.entry(num).or_insert(0) += 1;
            }
        }
    
        res
    }