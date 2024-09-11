fn main() {
    let args: Vec<String> = std::env::args().collect();
    let start: i32 = (&args[1]).parse().unwrap();
    let goal: i32 = (&args[2]).parse().unwrap();
    let output = Solution::min_bit_flips(start, goal);
    println!("{:?}", output);
}

struct Solution;

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        let mut min_bit_flips = 0;

        for bit_index in 0..32 {
            let start_bit = (start >> bit_index) & 1;
            let goal_bit = (goal >> bit_index) & 1;

            if start_bit != goal_bit {
                min_bit_flips += 1;
            }
        }
        return min_bit_flips;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let output = Solution::min_bit_flips(10, 7);
        assert_eq!(output, 3);
    }

    #[test]
    fn test2() {
        let output = Solution::min_bit_flips(3, 4);
        assert_eq!(output, 3);
    }
}