fn pivot_index(nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        let left_sum: i32 = nums[..i].iter().sum();
        let right_sum: i32 = nums[i + 1..].iter().sum();
        
        if left_sum == right_sum {
            return i as i32;
        }
    }
    return -1;
}

#[cfg(test)]
mod test{
    use crate::pivot_index;

    #[test]
    fn test() {
        let nums = vec![1,7,3,6,5,6];
        assert_eq!(pivot_index(nums), 3);
        let nums = vec![2,1,-1];
        assert_eq!(pivot_index(nums),0);
    }
}