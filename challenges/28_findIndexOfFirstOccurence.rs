struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut index: i32 = -1;
        let haystack_chars: Vec<char> = haystack.chars().collect();
        let needle_chars: Vec<char> = needle.chars().collect();

        for i in 0..haystack.len() {
            println!("i: {}", i);
            println!("haystack_chars[i]: {}", haystack_chars[i]);
            println!("needle_chars[i]: {}", needle_chars[i]);
            if index == -1 {
                println!("index: {}", index);
                if haystack_chars[i] == needle_chars[i]{
                    index = i as i32;
                    println!("first char match index: {}", index);
            }
            else {
                println!("index: {}", index);
                // condition de stop 
                if (i - index as usize)  == needle.len() {
                    println!("found a complete match: {}", i);
                    break;
                }

                if haystack_chars[i] != needle_chars[i] {
                    println!("no match, breaking inner loop");
                    index = -1;
                }
        }
        }
        return index as i32;
    }
}

fn main() {
    let haystack = String::from("hello");
    let needle = String::from("ll");
    let result = Solution::str_str(haystack, needle);
    println!("{}", result);
}