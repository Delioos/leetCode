struct State {
    current: String,
}

impl State {
    pub fn new(num_rows: i32) -> State {
        State {
            current: if num_rows == 1 { "full".to_string() } 
                     else if num_rows % 2 == 1 { "fullOdd".to_string() } 
                     else { "fullEven".to_string() },
        }
    }
    pub fn next(&mut self) {
        match self.current.as_str() {
            "fullOdd" => self.current = "mid".to_string(),
            "mid" => self.current = "fullOdd".to_string(),
            "fullEven" => self.current = "midBot".to_string(),
            "midBot" => self.current = "midTop".to_string(),
            "midTop" => self.current = "fullEven".to_string(),
            "full" => {},
            _ => panic!("Invalid state"),
        }
    }

    fn get(&self) -> String {
        self.current.clone()
    }
}

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut rows: Vec<String> = vec![String::new(); num_rows as usize];
        let mut state = State::new(num_rows);
        let mut row = 0;

        for c in s.chars() {
            match state.get().as_str() {
                "fullOdd" | "fullEven" => {
                    rows[row].push(c);
                    row += 1;
                    if row == num_rows as usize {
                        row -= 2;
                        state.next();
                    }
                },
                "mid" => {
                    rows[row].push(c);
                    row -= 1;
                    if row == 0 {
                        state.next();
                    }
                },
                "midBot" => {
                    rows[row].push(c);
                    row -= 1;
                    state.next();
                },
                "midTop" => {
                    rows[row].push(c);
                    row += 1;
                    if row == num_rows as usize - 1 {
                        state.next();
                    }
                },
                _ => panic!("Invalid state"),
            }
        }

        rows.into_iter().collect()
    }
}

fn main() {
    assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR".to_string());
}



