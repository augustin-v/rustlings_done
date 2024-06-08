    fn area(height:i32, length:i32) -> i32 {
        height * length
    }

    fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut res = 0;

        if height.len() == 2 {
            return height[left].min(height[right]);
        }

        while left < right {
            if height[left] > height[right] {
                if area(height[right], right as i32 - left as i32) > res {
                    res = area(height[right], right as i32 - left as i32)
                }
            }
            if height[right] >= height[left] {
                if area(height[left], right as i32 - left as i32) > res {
                    res = area(height[left], right as i32 - left as i32)
                }
            }
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        res
    }

#[cfg(test)]
mod test{
    use crate::max_area;

    #[test]
    fn basic() {
        let height = vec![1,8,6,2,5,4,8,3,7];
        assert_eq!(max_area(height), 49);
    }

    #[test]
    fn test() {
        let height = vec![1,1];
        assert_eq!(max_area(height), 1);
        let height = vec![2,1];
        assert_eq!(max_area(height), 1);
    }
}
