use std::cmp;

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut index = 0;
        let max_size = cmp::max(a.len(), b.len());

        let a: Vec<u8> = a
            .chars()
            .map(|char| char.to_digit(2).unwrap() as u8)
            .collect();
        let b: Vec<u8> = b
            .chars()
            .map(|char| char.to_digit(2).unwrap() as u8)
            .collect();

        let mut c = vec![0; max_size + 1];
        let mut carry = 0;

        while index < max_size || carry > 0 {
            let a_bit = if index < a.len() { a[a.len() - 1 - index] } else { 0 };
            let b_bit = if index < b.len() { b[b.len() - 1 - index] } else { 0 };

            let sum = a_bit + b_bit + carry;
            let c_index = c.len() - 1 - index; // Store the index in a local variable
            c[c_index] = sum % 2;
            carry = sum / 2;

            index += 1;
        }

        let result: String = c.into_iter().skip_while(|&x| x == 0).map(|x| (x + b'0') as char).collect();
        if result.is_empty() {
            return "0".to_string();
        }
        result
    }
}

fn main() {
    let a = "1010".to_string();
    let b = "1011".to_string();
    let result = Solution::add_binary(a, b);
    println!("{}", result);
}

