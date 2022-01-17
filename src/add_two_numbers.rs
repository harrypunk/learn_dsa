// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = None;

        let mut tail = &mut result;
        // (list1, list2, sum, carry)
        let mut temp = (l1, l2, 0, 0);
        loop {
            temp = match temp {
                (None, None, _, 0) => break,
                (None, None, _, _) => (None, None, 1, 0),
                (Some(node), None, _, carry) | (None, Some(node), _, carry) => {
                    let (sum1, carray1) = add(node.val, 0, carry);
                    (node.next, None, sum1, carray1)
                }
                (Some(n1), Some(n2), _, carry) => {
                    let (sum1, carray1) = add(n1.val, n2.val, carry);
                    (n1.next, n2.next, sum1, carray1)
                }
            };
            *tail = Some(Box::new(ListNode::new(temp.2)));
            tail = &mut tail.as_mut().unwrap().next;
        }

        result
    }
}

//add two digits (num, carry)
fn add(a: i32, b: i32, carry: i32) -> (i32, i32) {
    let sum = a + b + carry;
    let carry1 = if sum > 9 { 1 } else { 0 };
    (sum - carry1 * 10, carry1)
}
