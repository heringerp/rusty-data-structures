enum Node<T> {
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

    fn to_none(&mut self) {
        *self = Self::None
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
    }
}
