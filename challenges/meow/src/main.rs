struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // Handle empty needle case
        if needle.is_empty() {
            return 0;
        }

        let haystack_chars: Vec<char> = haystack.chars().collect();
        let needle_chars: Vec<char> = needle.chars().collect();
        
        // Check if needle is longer than haystack
        if needle.len() > haystack.len() {
            return -1;
        }

        // Check each possible starting position in haystack
        for i in 0..=haystack.len() - needle.len() {
            let mut found = true;
            // Compare needle with haystack substring
            for j in 0..needle.len() {
                if haystack_chars[i + j] != needle_chars[j] {
                    found = false;
                    break;
                }
            }
            if found {
                return i as i32;
            }
        }
        
        -1
    }
}

fn main() {
    let haystack = String::from("hello");
    let needle = String::from("ll");
    let result = Solution::str_str(haystack, needle);
    println!("{}", result);
}
