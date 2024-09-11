fn main() {
    let args: Vec<String> = std::env::args().collect();
    let output = Solution::detect_capital_use(args[1].clone());
    println!("{:?}", output);
}

struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut all_capitals = true;
        let mut all_not_capitals = true;
        let mut only_first_capital = true;

        for (i, c) in word.chars().enumerate() {
            if i == 0 {
                if c.is_ascii_uppercase() {
                    only_first_capital = true;
                    all_capitals = true;
                    all_not_capitals = false;
                } else {
                    only_first_capital = false;
                    all_capitals = false;
                    all_not_capitals = true;
                }
            } else {
                if c.is_ascii_uppercase() {
                    only_first_capital = false;
                    all_not_capitals = false;
                } else {
                    all_capitals = false;
                }
            }
        }
        return only_first_capital || all_not_capitals || all_capitals;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let output = Solution::detect_capital_use("USA".to_string());
        assert_eq!(output, true);
    }

    #[test]
    fn test2() {
        let output = Solution::detect_capital_use("FlaG".to_string());
        assert_eq!(output, false);
    }

    #[test]
    fn test3() {
        let output = Solution::detect_capital_use("test".to_string());
        assert_eq!(output, true);
    }

    #[test]
    fn test4() {
        let output = Solution::detect_capital_use("Nest".to_string());
        assert_eq!(output, true);
    }
}