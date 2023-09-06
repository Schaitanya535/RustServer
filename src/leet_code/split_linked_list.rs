// Definition for singly-linked list.
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

pub fn solve() {
    split_list_to_parts(None, 22);
}

pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
    let mut length = 0;
    let mut ans = Vec::new();
    let mut it = head.as_ref();
    while let Some(node) = it {
        length += 1;
        it = node.next.as_ref();
    }

    let (base_size, mut extra) = (length / k, length % k);
    let mut current = head;

    for _ in 0..k {
        let mut part_size = base_size + if extra > 0 { 1 } else { 0 };
        let mut dummy = Box::new(ListNode { val: 0, next: None });
        let mut tail = &mut dummy;

        while part_size > 0 {
            tail.next = current.take();
            tail = tail.next.as_mut().unwrap();
            current = tail.next.take();
            part_size -= 1;
        }

        ans.push(dummy.next.take());
        if extra > 0 {
            extra -= 1;
        }
    }
    ans
}
