use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input: Vec<i32> = serde_json::from_str(&args[1]).unwrap();
    let target: i32 = (&args[2]).parse().unwrap();

    let output = Solution::two_sum(input, target);
    println!("{:?}", output);
}

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (index, value) in nums.iter().enumerate() {
            map.insert(*value, index);
        }

        for (key, val) in map.iter() {
            let other = target - *key;
            if map.contains_key(&other) {
                let index1 = (*val) as i32;
                let index2 = *(map.get(&other).unwrap()) as i32;
                return vec![std::cmp::min(index1, index2) , std::cmp::max(index1, index2)];
            }
        }

        panic!("No solution found!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![2,7,11,15];
        let output = Solution::two_sum(input, 9);
        assert_eq!(output, [0,1]);
    }

    #[test]
    fn test2() {
        let input = vec![3,2,4];
        let output = Solution::two_sum(input, 6);
        assert_eq!(output, [1,2]);
    }

    #[test]
    fn test3() {
        let input = vec![3,3];
        let output = Solution::two_sum(input, 6);
        assert_eq!(output, [0,1]);
    }
}