fn main() {
    let args: Vec<String> = std::env::args().collect();
    let first_list: Vec<i32> = serde_json::from_str(&args[1]).unwrap();
    let second_list: Vec<i32> = serde_json::from_str(&args[2]).unwrap();

    let output = Solution::add_two_numbers(Solution::from_vec(first_list), Solution::from_vec(second_list));
    println!("{:?}", Solution::to_vec(output));
}

struct Solution;

impl Solution {
  pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut digits = Vec::new();
    let mut over = 0;
    let mut l1_current = l1;
    let mut l2_current = l2;
    loop {
      if l1_current.is_some() && l2_current.is_some() {
        let l1_node = l1_current.clone().unwrap();
        let l2_node = l2_current.clone().unwrap();
        let sum = l1_node.val + l2_node.val + over;
        over = sum / 10;
        let val = sum % 10;
        digits.push(val);

        l1_current = l1_node.next;
        l2_current = l2_node.next;
        continue;
      }
      
      if l1_current.is_none() && l2_current.is_none() {
        if over > 0 {
          digits.push(over);
        }

        return Self::reverse(Self::from_vec(digits));
      }

      if l1_current.is_some() && l2_current.is_none() {
        let l1_node = l1_current.clone().unwrap();
        let sum = l1_node.val + over;
        over = sum / 10;
        let val = sum % 10;
        digits.push(val);

        l1_current = l1_node.next;
        continue;
      }

      if l1_current.is_none() && l2_current.is_some() {
        let l2_node = l2_current.clone().unwrap();
        let sum = l2_node.val + over;
        over = sum / 10;
        let val = sum % 10;
        digits.push(val);
        l2_current = l2_node.next;
        continue;
      }
    }

    return Self::reverse(Self::from_vec(digits));
  }

  fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut vec = Self::to_vec(head);
    return Self::from_vec(vec);
  }

  fn from_vec(vector: Vec<i32>) -> Option<Box<ListNode>> {
    let mut node: Option<Box<ListNode>> = None;
  
    for (i, digit) in vector.iter().enumerate() {
        if i == 0 {
          node = Some(Box::new(ListNode::new(*digit)));
        } else {
          node = Some(Box::new(ListNode {
              val: *digit,
              next: node,
          }));
        }
    }
  
    return node;
  }
  
  fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head;
    while let Some(node) = current {
      result.push(node.val);
      current = node.next;
    }
  
    return result;
  }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let output = Solution::add_two_numbers(Solution::from_vec(vec![2,4,3]), Solution::from_vec(vec![5,6,4]));
        assert_eq!(Solution::to_vec(output), [7,0,8]);
    }

    #[test]
    fn test2() {
        let output = Solution::add_two_numbers(Solution::from_vec(vec![0]), Solution::from_vec(vec![0]));
        assert_eq!(Solution::to_vec(output), [0]);
    }

    #[test]
    fn test3() {
        let output = Solution::add_two_numbers(Solution::from_vec(vec![9,9,9,9,9,9,9]), Solution::from_vec(vec![9,9,9,9]));
        assert_eq!(Solution::to_vec(output), [8,9,9,9,0,0,0,1]);
    }

    #[test]
    fn test4() {
        let output = Solution::add_two_numbers(Solution::from_vec(vec![1,1,1]), Solution::from_vec(vec![2,2,2]));
        assert_eq!(Solution::to_vec(output), [3,3,3]);
    }

    #[test]
    fn test5() {
        let output = Solution::add_two_numbers(Solution::from_vec(vec![1,1]), Solution::from_vec(vec![2]));
        assert_eq!(Solution::to_vec(output), [3,1]);
    }
}