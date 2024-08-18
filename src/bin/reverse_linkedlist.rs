#[derive(Debug, PartialEq, Eq, Clone)]
// Define the ListNode structure with Debug, PartialEq, Eq, and Clone traits derived.
// Debug allows for printing the structure, PartialEq and Eq are used for comparisons,
// and Clone allows for deep copying of ListNode instances.
pub struct ListNode {
    pub val: i32, // The value stored in the node.
    pub next: Option<Box<ListNode>>, // A link to the next node in the list, wrapped in an Option.
}

// Implementing methods for the ListNode structure.
impl ListNode {
    #[inline]
    // This is an inline function, suggesting the compiler should try to optimize it by inlining.
    fn new(val: i32) -> Self {
        // Constructor method for creating a new ListNode with a given value.
        // The next field is set to None, meaning this node does not initially point to another node.
        ListNode {
            val, 
            next: None,
        }
    }
}

pub struct Solution {}

// Implement the Solution struct, where we define our reverse_list method.
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // This method takes an Option containing a Box pointing to the head of the list.
        // It returns an Option containing a Box pointing to the new head of the reversed list.

        let (mut prev, mut curr) = (None, head);
        // Initialize two mutable variables:
        // prev will hold the previous node (starting as None because the new tail will point to None),
        // curr will be the current node we are examining (starting as the head of the list).

        while let Some(mut node) = curr {
            // Enter the loop if curr is Some(Box<ListNode>), meaning there is a node to process.
            // The loop continues until curr becomes None, indicating the end of the list.

            curr = node.next;
            // Move curr to the next node in the list. This advances the loop.

            node.next = prev;
            // Reverse the current node's link. Instead of pointing to the next node in the original list,
            // it now points to the previous node (initially None, then the node processed in the last iteration).

            prev = Some(node);
            // Move prev to the current node. This sets up the next iteration, where the current node
            // will become the previous node.
        }
        prev
        // After the loop completes, prev holds the new head of the reversed list.
        // Return prev as the new head.
    }
}

fn main() {
    println!("starting");
    // A placeholder main function to illustrate where the program would begin execution.
    // This does not do anything meaningful in this context, but is necessary for a complete Rust program.
}

