// Полезные методы:
// - https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.as_mut
// - https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.as_ref
// - https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.take
// - https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.map
use std::mem;
pub struct List<T> {
    head: Link<T>,
}

// Что будет, если убрать здесь Box? Попробуйте.
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node { 
            elem: elem, 
            next: mem::replace(&mut self.head, None)//??
    });
    
        self.head = Some(node); 
        
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, None) {//??
            Some(node)=> {
                self.head = node.next;
                Some(node.elem)
            },
            None => None
        }

    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|h| &h.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|h| &mut h.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(node) = current {
            current = node.next;
        }
        println!("List droped here")
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
}