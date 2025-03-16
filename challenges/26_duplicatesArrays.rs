
struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut last_value: i32 = 999;
        let mut c: i32 = 0;
        for n in nums.clone().iter_mut() {
            if *n == last_value {
                nums.remove(c as usize);
                c -= 1;
            }
            last_value = *n;
            c += 1;
        }
        return nums.len() as i32;
    }
}



fn main() {
    let mut nums = vec![-1, 0, 0, 0, 0, 3, 3];
    let result = Solution::remove_duplicates(&mut nums);
    println!("Result: {}", result);
}

