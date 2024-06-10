    fn is_vowel(c: char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }
    
    fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        
        // Initial count of vowels in the first window
        let mut count = 0;
        for i in 0..k {
            if is_vowel(chars[i]) {
                count += 1;
            }
        }
        
        let mut max_count = count;
        
        // Slide the window over the string
        for i in k..len {
            if is_vowel(chars[i - k]) {
                count -= 1;
            }
            if is_vowel(chars[i]) {
                count += 1;
            }
            max_count = max_count.max(count);
        }
        
        max_count
    }