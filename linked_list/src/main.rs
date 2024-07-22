mod linked_list {
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    pub struct Node<T> {
        pub val: T,
        pub next: InnerRef<T>,
    }
    
    impl<T> Node<T> {
        pub fn new(val: T, next: InnerRef<T>) -> Node<T> {
            Node {
                val,
                next,
            }
        }

        pub fn val(&self) -> T
            where T: std::clone::Clone {
            self.val.clone()
        }
    }
    
    type InnerRefType<T> = RefCell<Weak<Option<Node<T>>>>;
    type OuterRefType<T> = RefCell<Rc<Option<Node<T>>>>;
    
    pub struct InnerRef<T> (InnerRefType<T>);
    pub struct OuterRef<T> (OuterRefType<T>);
    
    impl<T> InnerRef<T> {
        pub fn new() -> InnerRef<T> {
            InnerRef(RefCell::new(Weak::new()))
        }
    }
    
    impl<T> OuterRef<T> {
        pub fn new(node: Node<T>) -> OuterRef<T> {
            OuterRef(RefCell::new(Rc::new(Some(node))))
        }

        pub fn empty() -> OuterRef<T> {
            OuterRef(RefCell::new(Rc::new(None)))
        }
    }
    
    pub struct List<T> {
        head: OuterRef<T>,
    }
    
    impl<T> List<T> {
        pub fn new(_head: Option<OuterRef<T>>) -> List<T> {
            match _head {
                Some(head) => List { head },
                None => {
                    panic!("Empty Reference of List !!!");
                },
            }
        }
    
        pub fn init(val: T) -> List<T> {
            List { 
                head: OuterRef::new(Node::new(val, InnerRef::new())),
            }
        }

        pub fn empty() -> List<T> {
            List {
                head: OuterRef::empty(),
            }
        }
        
        pub fn push_ahead(&self, node: Node<T>)
            where T: std::fmt::Display + std::clone::Clone {
            *node.next.0.borrow_mut() = Rc::downgrade(&self.head.0.borrow());
            *self.head.0.borrow_mut() = Rc::new(Some(node));        
        }

        pub fn insert(&self, val: T)
            where T: std::fmt::Display + std::clone::Clone {
            Self::push_ahead(&self, Node::new(val, InnerRef::new()));
        }
    }
    
    impl<T: std::fmt::Display> std::fmt::Display for List<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut iter = Rc::clone(&self.head.0.borrow());
            let mut empty: bool = true;
            
            loop {
                match Rc::clone(&iter).as_ref() {
                    Some(it) => {  // &Node<T>
                        if empty {
                            // the list before read the first one is also empty
                            write!(f, "[{}", it.val)?; 
                            empty = false;
                        } else {
                            write!(f, ", {}", it.val)?;
                        }
                        iter = it.next.0.borrow().upgrade().unwrap();
                    },
                    None => {
                        if !empty  {
                            write!(f, "]")?;
                        }
                        break;
                    },
                }
            }
            Ok(())
        }
    }
}

fn main() {
    let list = linked_list::List::init(1);
    list.insert(2);
    list.insert(3);
    list.insert(4);
    println!("{}", list);
}

    