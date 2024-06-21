use std::collections::{HashMap, HashSet};


fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    let mut freq1 = HashMap::new();
    let mut freq2 = HashMap::new();

    for ch in word1.chars() {
        *freq1.entry(ch).or_insert(0) += 1;
    }

    for ch in word2.chars() {
        *freq2.entry(ch).or_insert(0) += 1;
    }

    // Check if both words have the same set of unique characters
    let set1: HashSet<&char> = freq1.keys().collect();
    let set2: HashSet<&char> = freq2.keys().collect();
    if set1 != set2 {
        return false;
    }

    // Check if both words have the same frequencies of characters
    let mut values1: Vec<i32> = freq1.values().cloned().collect();
    let mut values2: Vec<i32> = freq2.values().cloned().collect();
    
    values1.sort();
    values2.sort();

    values1 == values2
}