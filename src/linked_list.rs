pub enum Node<T> {
    None,
    Tail { value: T },
    Link { value: T, next: Box<Self> }, 
}

impl<T> Node<T> where T: Copy {
    pub fn push(&mut self, value: T) {
        match self {
            Self::None => { self.to_tail(value) },
            Self::Tail { value:_ } => { self.to_link(value) },
            Self::Link { value:_, next } => { next.push(value) },
        }
    }

    pub fn lpush(&mut self, value: T) {
        match self {
            Self::None => { self.to_tail(value) },
            Self::Tail { value:_ } => {  },
            Self::Link { value:_, next:_ } => {
                let old_head = std::mem::replace(self, Node::None);
                let mut n = Node::Link{ value, next: Box::new(old_head)};
                std::mem::swap(self, &mut n);
            },
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        if i == 0 {
            return match self {
                Self::Tail { value } => Some(value),
                Self::Link { value, next:_ } => Some(value),
                Self::None => None,
            }
        } else {
            return match self {
                Self::Tail { value:_ } => None,
                Self::Link { value:_, next } => next.get(i - 1),
                Self::None => None,
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self::None => None,
            Self::Tail { value } => {
                let value = *value;
                self.to_none();
                Some(value)
            },
            Self::Link { value, next } => {
                let mut n = Box::new(Node::None);
                let item = *value;
                std::mem::swap(next, &mut n);
                self.to_next(*n);
                Some(item)
            },
        }
    }

    pub fn rpop(&mut self) -> Option<T> {
        match self {
            Self::None => None,
            Self::Tail { value } => {
                let value = *value;
                self.to_none();
                Some(value)
            },
            Self::Link { value:_, next } => {
                match **next {
                    Self::Tail { value: tailvalue } => { self.to_tail_without_value(); Some(tailvalue) },
                    Self::Link { value:_, next:_ } => next.rpop(),
                    _ => panic!("Link should never end in None")
                }
            }
        }
    }

    fn to_none(&mut self) {
        *self = Self::None
    }

    fn to_tail_without_value(&mut self) {
        *self = match self {
            Self::Link {value, next:_} => Self::Tail { value: *value },
            _ => panic!("Cannot convert tail to tail"),
        }
    }

    fn to_tail(&mut self, value: T) {
        *self = match self {
            Self::None => Self::Tail { value },
            Self::Link {value:_, next:_} => Self::Tail { value },
            _ => panic!("Cannot convert tail to tail"),
        }
    }

    fn to_link(&mut self, new_value: T) {
        *self = match self {
            Self::Tail { value } => Self::Link { value: *value, next: Box::new(Self::Tail { value: new_value } ) },
            _ => panic!("Cannot convert tail to tail"),
        }
    }

    fn to_next(&mut self, next: Node<T>) {
        *self = next; 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
    }

    #[test]
    fn test_add_single_element() {
        let mut list = Node::None;
        list.push(1);
        assert_eq!(1, list.pop().unwrap());
    }

    #[test]
    fn test_add_multiple_elements() {
        let mut list = Node::None;
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(1, list.pop().unwrap());
        assert_eq!(2, list.pop().unwrap());
        assert_eq!(3, list.pop().unwrap());
    }

    #[test]
    fn test_push_left() {
        let mut list = Node::None;
        list.push(1);
        list.push(2);
        list.lpush(3);
        assert_eq!(3, list.pop().unwrap());
        assert_eq!(1, list.pop().unwrap());
        assert_eq!(2, list.pop().unwrap());
    }

    #[test]
    fn test_pop_right() {
        let mut list = Node::None;
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(3, list.rpop().unwrap());
        assert_eq!(2, list.rpop().unwrap());
        assert_eq!(1, list.rpop().unwrap());
    }

    #[test]
    fn test_get() {
        let mut list = Node::None;
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(3, *list.get(2).unwrap());
        assert_eq!(1, *list.get(0).unwrap());
        assert_eq!(2, *list.get(1).unwrap());
    }
}
