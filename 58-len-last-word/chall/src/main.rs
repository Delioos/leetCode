struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let l = s.split(" ").filter(|&word| !word.is_empty()).collect::<Vec<&str>>();
        l[l.len() - 1].len() as i32
    }
}

fn main() {
    let s = String::from("   fly me   to   the moon  ");
    println!("{}", Solution::length_of_last_word(s));
}