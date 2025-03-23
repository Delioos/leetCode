/*
* Given a non-negative integer x, return the square root of x rounded down to the nearest integer. The returned integer should be non-negative as well.
* You must not use any built-in exponent function or operator.

*/
struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 || x == 1 {
            return x;
        }
        
        let mut left: i64 = 1;
        let mut right = x as i64;
        let mut ans: i64 = 0;
        
        while left <= right {
            let mid = left + (right - left) / 2;
            
            if mid * mid <= right {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        ans as i32
    }
}

fn main() {
    println!("{}", Solution::my_sqrt(2147395600));
}
