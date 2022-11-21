struct Linked_List<T> {
    length: usize,
    head: Option<Node<T>>
}

impl<T> Linked_List<T> {
    fn new() -> Self {
        Linked_List{
            length: 0,
            head: None,
        }
    }

    fn len(self) -> usize {
        self.length
    }

    fn add(&mut self, value: T) {
        match &mut self.head {
            None => { 
                self.head = Some(
                    Node{value, next: None}) },
            Some(x) => {
                let mut current = x;
                while current.next.is_some() {
                   current = &mut *current.next.expect("Should never happen"); 
                }
                current.next = Some(
                    Box::new(Node{value, next: None}))}
        }
        self.length += 1;
    }
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let list: Linked_List<u32> = Linked_List::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_add_single_element() {
        let mut list = Linked_List::new();
        list.add(1);
        assert_eq!(list.len(), 0);
    }
}
