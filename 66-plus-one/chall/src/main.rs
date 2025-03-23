struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits; 
        let mut index = digits.len();

        let mut carry= true;

        while carry && index > 0 {
            index-=1; 
            digits[index] += 1;
            if digits[index] == 10 {
                digits[index] = 0;    

                if index ==  0 {
                    // add a new col and update left value 
                    digits.push(0);
                    digits[index] = 1;
                }
            }
            else {
                carry = false;
            }
        }
        
        digits
        
    }
}

fn main() {
    let digits = vec![9,9,9];
    let result = Solution::plus_one(digits);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        let digits = vec![1,2,3];
        let result = Solution::plus_one(digits);
        assert_eq!(result, vec![1,2,4]);
    }   

    #[test]
    fn test_plus_one_with_carry() {
        let digits = vec![9,9,9];
        let result = Solution::plus_one(digits);
        assert_eq!(result, vec![1,0,0,0]);
    }

    #[test]
    fn test_plus_one_with_multiple_carries() {
        let digits = vec![9,9,9,9];
        let result = Solution::plus_one(digits);
        assert_eq!(result, vec![1,0,0,0,0]);
    }
    
    
}
