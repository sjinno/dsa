#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut reversed: Option<Box<ListNode>> = None;
    let mut current = head;

    while let Some(mut node) = current {
        current = node.next.take();
        node.next = reversed;
        reversed = Some(node);
    }

    reversed
}
