fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut input: Vec<i32> = serde_json::from_str(&args[1]).unwrap();

    let output = Solution::remove_duplicates(&mut input);
    println!("{:?}", output);
}

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let length = nums.len();
        if length < 2 {
            return length as i32;
        }

        let mut current_index = 1;
        let mut previous_different_index = 0;
        let mut previous_different_value = nums[previous_different_index];
        let mut unique_count = 1;

        while current_index < length {
            let current_value = nums[current_index];

            if current_value != previous_different_value {
                nums[previous_different_index + 1] = current_value;
                previous_different_index = previous_different_index + 1;
                previous_different_value = current_value;
                unique_count += 1;
            }

            current_index += 1;
        }

        return unique_count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut input = vec![1,1,2];
        let output = Solution::remove_duplicates(&mut input);
        assert_eq!(output, 2);
    }

    #[test]
    fn test2() {
        let mut input = vec![0,0,1,1,1,2,2,3,3,4];
        let output = Solution::remove_duplicates(&mut input);
        assert_eq!(output, 5);
    }

    #[test]
    fn test3() {
        let mut input = vec![5];
        let output = Solution::remove_duplicates(&mut input);
        assert_eq!(output, 1);
    }

    #[test]
    fn test4() {
        let mut input = vec![];
        let output = Solution::remove_duplicates(&mut input);
        assert_eq!(output, 0);
    }
}