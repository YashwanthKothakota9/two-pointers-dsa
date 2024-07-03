use std::fmt::Display;

#[derive(Clone)]
struct LinkedListNode {
    data: i32,
    next: Option<Box<LinkedListNode>>,
}

impl LinkedListNode {
    fn new(d: i32) -> Self {
        LinkedListNode {
            data: d,
            next: None,
        }
    }
}

#[derive(Clone)]
struct LinkedList {
    head: Option<Box<LinkedListNode>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn insert_node_at_head(&mut self, mut node: Box<LinkedListNode>) {
        node.next = self.head.take();
        self.head = Some(node);
    }

    fn create_linked_list(&mut self, vec: Vec<i32>) {
        for value in vec.into_iter().rev() {
            let node = Box::new(LinkedListNode::new(value));
            self.insert_node_at_head(node);
        }
    }

    fn get_length(&self) -> usize {
        let mut length = 0;
        let mut temp = &self.head;
        while let Some(ref node) = temp {
            length += 1;
            temp = &node.next;
        }
        length
    }

    fn get_node(&self, pos: usize) -> Option<&LinkedListNode> {
        let mut p = 0;
        let mut temp = &self.head;
        while let Some(ref node) = temp {
            if p == pos {
                return Some(node);
            }
            p += 1;
            temp = &node.next;
        }
        None
    }

    fn print_list_with_forward_arrow(&self) {
        let mut temp = &self.head;
        while let Some(ref node) = temp {
            print!("{}", node.data);
            temp = &node.next;
            if temp.is_some() {
                print!(" → ");
            } else {
                print!(" → NULL");
            }
        }
        println!();
    }
}

impl Display for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        let mut temp = &self.head;

        while let Some(ref node) = temp {
            result.push_str(&node.data.to_string());
            temp = &node.next;
            if temp.is_some() {
                result.push_str(", ");
            }
        }

        write!(f, "{}", result)
    }
}

fn remove_nth_last_node(
    head: Option<Box<LinkedListNode>>,
    n: usize,
) -> Option<Box<LinkedListNode>> {
    let mut head = head.unwrap();
    let mut fast = head.clone();
    let mut slow = head.as_mut();

    let mut meet_end = false;
    for _ in 0..n {
        if fast.next.is_none() {
            meet_end = true;
            break;
        }
        fast = fast.next.unwrap();
    }

    if meet_end {
        if n == 1 {
            return None;
        }
        return head.next;
    }

    while let Some(node) = fast.next {
        slow = slow.next.as_mut().unwrap();
        fast = node;
    }

    slow.next = slow.next.as_mut().unwrap().next.clone();
    Some(head)
}

pub(crate) fn main() {
    let inputs = vec![
        vec![23, 89, 10, 5, 67, 39, 70, 28],
        vec![34, 53, 6, 95, 38, 28, 17, 63, 16, 76],
        vec![288, 224, 275, 390, 4, 383, 330, 60, 193],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![69, 8, 49, 106, 116, 112, 104, 129, 39, 14, 27, 12],
    ];

    let n = vec![4, 1, 6, 9, 11];

    for (i, input) in inputs.iter().enumerate() {
        let mut linkedlist = LinkedList::new();
        linkedlist.create_linked_list(input.clone());
        println!("{}. Linked List:        ", i + 1);
        linkedlist.print_list_with_forward_arrow();
        println!("n = {}", n[i]);
        println!("Updated Linked List:   ");
        linkedlist.print_list_with_forward_arrow();
        println!("{}", "-".repeat(100));
    }
}
