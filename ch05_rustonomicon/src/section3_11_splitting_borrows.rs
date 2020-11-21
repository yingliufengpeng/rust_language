
//!
//! # Splitting Borrows
//!
//!
//!
//!



#[cfg(test)]
mod tests {
    use std::mem;
    use std::collections::VecDeque;

    struct Foo {
        a: i32,
        b: i32,
        c: i32,
    }

    type Link<T> = Option<Box<Node<T>>>;

    struct Node<T> {
        elem: T,
        next: Link<T>,
    }

    pub struct LinkedList<T> {
        head: Link<T>,
    }

    impl<T> LinkedList<T> {
        fn iter_mut(&mut self) -> IterMut<T> {
            IterMut(self.head.as_mut().map(|node| &mut **node))
        }
    }

    pub struct IterMut<'a, T: 'a>(Option<&'a mut Node<T>>);

    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.take().map(|node| {
                self.0 = node.next.as_mut().map(|node| &mut **node);
                &mut node.elem
            })
        }
    }

    pub struct IterMut2<'a, T: 'a>(&'a mut[T]);

    impl<'a, T> Iterator for IterMut2<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            let slice = mem::replace(&mut self.0, &mut []);
            if slice.is_empty() { return None; }

            let (l, r) = slice.split_at_mut(1);
            self.0 = r;
            l.get_mut(0)
        }
    }

    // impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    //     fn next_back(&mut self) -> Option<Self::Item> {
    //         let slice = mem::replace(&mut self.0, &mut []);
    //         if slice.is_empty() { return None; }
    //
    //         let new_len = slice.len() - 1;
    //         let (l, r) = slice.split_at_mut(new_len);
    //         self.0 = l;
    //         r.get_mut(0)
    //     }
    // }

    type Link3<T> = Option<Box<Node3<T>>>;

    struct Node3<T> {
        elem: T,
        left: Link3<T>,
        right: Link3<T>,
    }

    pub struct Tree<T> {
        root: Link3<T>,
    }

    struct NodeIterMut<'a, T: 'a> {
        elem: Option<&'a mut T>,
        left: Option<&'a mut Node3<T>>,
        right: Option<&'a mut Node3<T>>,
    }

    enum State<'a, T: 'a> {
        Elem(&'a mut T),
        Node3(&'a mut Node3<T>),
    }

    pub struct IterMut3<'a, T: 'a>(VecDeque<NodeIterMut<'a, T>>);

    impl<T> Tree<T> {
        pub fn iter_mut(&mut self) -> IterMut3<T> {
            let mut deque = VecDeque::new();
            self.root.as_mut().map(|root| deque.push_front(root.iter_mut()));
            IterMut3(deque)
        }
    }

    impl<T> Node3<T> {
        pub fn iter_mut(&mut self) -> NodeIterMut<T> {
            NodeIterMut {
                elem: Some(&mut self.elem),
                left: self.left.as_mut().map(|node| &mut **node),
                right: self.right.as_mut().map(|node| &mut **node),
            }
        }
    }


    impl<'a, T> Iterator for NodeIterMut<'a, T> {
        type Item = State<'a, T>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.left.take() {
                Some(node) => Some(State::Node3(node)),
                None => match self.elem.take() {
                    Some(elem) => Some(State::Elem(elem)),
                    None => match self.right.take() {
                        Some(node) => Some(State::Node3(node)),
                        None => None,
                    }
                }
            }
        }
    }

    // impl<'a, T> DoubleEndedIterator for NodeIterMut<'a, T> {
    //     fn next_back(&mut self) -> Option<Self::Item> {
    //         match self.right.take() {
    //             Some(node) => Some(State::Node3(node)),
    //             None => match self.elem.take() {
    //                 Some(elem) => Some(State::Elem(elem)),
    //                 None => match self.left.take() {
    //                     Some(node) => Some(State::Node3(node)),
    //                     None => None,
    //                 }
    //             }
    //         }
    //     }
    // }

    impl<'a, T> Iterator for IterMut3<'a, T> {
        type Item = &'a mut T;
        fn next(&mut self) -> Option<Self::Item> {
            loop {
                match self.0.front_mut().and_then(|node_it| node_it.next()) {
                    Some(State::Elem(elem)) => return Some(elem),
                    Some(State::Node3(node)) => self.0.push_front(node.iter_mut()),
                    None => if let None = self.0.pop_front() { return None },
                }
            }
        }
    }
    //
    // impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    //     fn next_back(&mut self) -> Option<Self::Item> {
    //         loop {
    //             match self.0.back_mut().and_then(|node_it| node_it.next_back()) {
    //                 Some(State::Elem(elem)) => return Some(elem),
    //                 Some(State::Node(node)) => self.0.push_back(node.iter_mut()),
    //                 None => if let None = self.0.pop_back() { return None },
    //             }
    //         }
    //     }
    // }
    #[test]
    fn test_001() {
        let mut x = Foo {a: 0, b: 0, c: 0};
        let a = &mut x.a;
        let b = &mut x.b;
        let c = &x.c;
        *b += 1;
        let c2 = &x.c;
        *a += 10;
        println!("{} {} {} {}", a, b, c, c2);

    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
