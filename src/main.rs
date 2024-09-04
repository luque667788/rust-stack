use std::any::Any;


struct Thing<T> {
    data: T,
    next: Option<Box<Thing<T>>>,
}

impl<T> Thing<T> {
    fn new(data: T) -> Thing<T> {
        Thing { data, next: None }
    }
}

struct List<T> {
    head: Option<Box<Thing<T>>>,
    count: u32,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List {
            head: None,
            count: 0,
        }
    }

    fn push(&mut self, data: T) {
        let mut new_item = Box::new(Thing::new(data));
        new_item.next = self.head.take(); // take() is used to move the value out of the Option and replace with None
                                          // the self.head is now equal None
        self.head = Some(new_item);
        self.count += 1;
    }

    fn pop(&mut self) -> Option<T> {
        let result = self.head.take();

        match result {
            None => {
                self.head = None; //not really necessary
                return None;
            }
            Some(item) => {
                self.head = item.next;
                self.count -= 1;
                return Some(item.data);
            }
        };
    }
}

fn main() {
    print!("Hello, world!\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut next = match list.head {
            Some(item) => Some(item),
            None => {
                panic!("No head! should have a head!");
            }
        };
        let mut c = 0;
        loop {
            match next {
                Some(item) => {
                    println!("Data: {}", item.data);
                    assert_eq!(item.data, 3 - c);
                    next = item.next;
                }
                None => break,
            }
            c += 1;
        }
    }

    #[test]
    fn test_pop() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
