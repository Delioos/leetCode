
```rs
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
```


nothing crazy but feels gr8 to practice. I really wanna become fluent in rust so instead of using dirty `println!()`for debugging session I learn how to use `rust-llb` with these two simple commands:

`rustc -g challenges/26_duplicatesArrays.rs`

& 
(could have used gbd too)
`rust-lldb ./26_duplicatesArrays`

#### cheatsheet for debugger 
Set a breakpoint: b remove_duplicates
Run the program: r
Step through code: n (next) or s (step into)
Print variables: p nums or p last_value
Continue execution: c
Quit debugger: q


# improvements
I know that my solution isn t really good looking and could be more functional programming oriented when I saw other codes like this : 
```rs
use std::collections::HashSet;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut used: Vec<i32> = Vec::<i32>::new();
        let mut write_index: usize = 0;
        (0..nums.len()).for_each(|read_index| {
            let i: i32 = nums[read_index];
            if !used.contains(&i) {
                nums[write_index] = i;
                used.push(i);
                write_index += 1;
            }
        });
        write_index as i32
    }
}
```

or simpler like this : 
```rs
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut index : usize = 0;
        for i in 1 .. nums.len(){
            if nums[i] != nums[index]{
                index +=1 ;
                nums[index] = nums[i];
            }
        } 
       ( index + 1 )as i32

    }
}
```

but I'm still happy to progress :)