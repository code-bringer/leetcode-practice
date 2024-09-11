fn main() {
    let args: Vec<String> = std::env::args().collect();
    let number: i32 = (&args[1]).parse().unwrap();

    let output = Solution::is_palindrome(number);
    println!("{:?}", output);
}

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        if x >= 0  && x <= 9 {
            return true;
        }

        let mut digits = Vec::new();
        let mut number = x;
        while number / 10 > 0 {
            let digit = number % 10;
            digits.push(digit);
            number = number / 10;
        }
        digits.push(number);

        let half_digit_count = digits.len() / 2;
        let mut begin_index: usize = 0;
        let mut end_index = digits.len() - 1;

        while begin_index <= half_digit_count {
            if digits[begin_index] != digits[end_index] {
                return false;
            }

            begin_index += 1;
            end_index -= 1;
        }
        
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let output = Solution::is_palindrome(121);
        assert_eq!(output, true);
    }

    #[test]
    fn test2() {
        let output = Solution::is_palindrome(-121);
        assert_eq!(output, false);
    }

    #[test]
    fn test3() {
        let output = Solution::is_palindrome(10);
        assert_eq!(output, false);
    }

    #[test]
    fn test4() {
        let output = Solution::is_palindrome(123454321);
        assert_eq!(output, true);
    }

    #[test]
    fn test5() {
        let output = Solution::is_palindrome(1001);
        assert_eq!(output, true);
    }

    #[test]
    fn test6() {
        let output = Solution::is_palindrome(1000);
        assert_eq!(output, false);
    }
}