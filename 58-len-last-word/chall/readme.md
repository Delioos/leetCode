https://leetcode.com/problems/length-of-last-word/solutions/6558142/0ms-runtime-by-delioos-evw3/
# Intuition
knew rust is well craftd so I dug the libs to find some functionnal things.
initially tried to exploit the iterator split whitespace with something like 

let mut iter = s.split_whitespace();
match iter.next_back() {
    Some(word) => word.len() as i32,
    _ => -1 
};

but split whitespace isn t available here apperently. Moreover, this other solution is maybe a bit more straightforward and easy to understand (we filter out empty words return the len of the last. thats all)

# Approach
meow 

# Complexity
![image.png](https://assets.leetcode.com/users/images/a0bc2deb-4589-42ba-94f2-08d9c58fc44e_1742450321.4629068.png)

O(n) goes brr

# Code
```rust []


impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let l= s.split(" ").filter(|&word| !word.is_empty()).collect::<Vec<&str>>();
        l[l.len() - 1].len() as i32
    }
}

```