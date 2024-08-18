type List = Option<Box<ListNode>>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ListNode {
    pub val: i32,
    pub next: List 
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


struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>> ) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;
        let mut carry = 0;

        let mut p = l1;
        let mut q = l2;

        while p.is_some() || q.is_some() {

            let x = p.as_ref().map_or(0, |node| node.val);
            let y = q.as_ref().map_or(0, |node| node.val);

            let sum = carry + x + y;
            carry = sum / 10;

            // create new node
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();

        } dummy_head.next
    }
    
}


fn main() {
    println!("started add_two_numbers");

    let list_1 = 4, 6, 8; 

    let list_2 = 2, 8, 5; 

    let result = Solution::add_two_numbers(list_1, list_2);
}
