#![allow(dead_code)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val,
    }
  }
}

// T: O(max{m,n}), S: O(max{m,n})
pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>)
                       -> Option<Box<ListNode>> {
  let mut answer = Some(Box::new(ListNode::new(0)));
  let mut ans = &mut answer;
  let mut carry = false;
  let mut is_unit_digit = true;

  while l1.is_some() || l2.is_some() || carry {
    let val = {
      let mut val = match &l1 {
        Some(node) => {
          let x = node.val;
          l1 = l1.unwrap().next;
          x
        }
        None => 0
      } + match &l2 {
        Some(node) => {
          let x = node.val;
          l2 = l2.unwrap().next;
          x
        }
        None => 0
      };
      if carry {
        val += 1;
        carry = false;
      }
      if val >= 10 {
        carry = true;
        val -= 10;
      }
      val
    };

    if is_unit_digit {
      ans.as_mut().unwrap().val = val;
      is_unit_digit = false;
    } else {
      ans.as_mut().unwrap().next = Some(Box::new(ListNode::new(val)));
      ans = &mut ans.as_mut().unwrap().next;
    }
  }

  answer
}


#[test]
fn test() {
  {
    let l1 = ListNode {
      val: 2,
      next: Some(Box::new(
        ListNode {
          val: 4,
          next: Some(Box::new(
            ListNode { val: 3, next: None }
          )),
        }
      )),
    };
    let l2 = ListNode {
      val: 5,
      next: Some(Box::new(
        ListNode {
          val: 6,
          next: Some(Box::new(
            ListNode { val: 4, next: None }
          )),
        }
      )),
    };
    let l3 = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));

    assert_eq!(
      Some(Box::new(ListNode {
        val: 7,
        next: Some(Box::new(
          ListNode {
            val: 0,
            next: Some(Box::new(
              ListNode { val: 8, next: None }
            )),
          }
        )),
      })),
      l3);
  }
  {
    let l1 = ListNode {
      val: 5,
      next: None,
    };
    let l2 = ListNode {
      val: 5,
      next: None,
    };
    let l3 = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));

    assert_eq!(
      Some(Box::new(ListNode {
        val: 0,
        next: Some(Box::new(
          ListNode {
            val: 1,
            next: None,
          }
        )),
      })),
      l3);
  }
}
