#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
    let mut result = 0;
    while let Some(node) = head {
        result = result * 2 + node.val;
        head = node.next;
    }
    result
}
