https://leetcode.com/problems/remove-element/solutions/6548047/less-is-more-by-delioos-fbtq

Intuition
none, I tried to do some crazy functional slop at first like
nums.iter().filter(|&x| x != &val).collect::Vec<&i32>().len() as i32
but I don't actually modify the nums list we just realloc a new one I think. So I tried a more straightforward approach.

Approach
just 1 var to keep track of gud values and store a count of final non pattern matching values

Complexity
Time complexity:
image.png

O(n) type shi

Space complexity:
image.png
O(1) type shi

Code
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;
        
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }
        
        k as i32
    }
}
