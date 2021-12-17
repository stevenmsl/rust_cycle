use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    /* b points to a */
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    if let Some(link) = a.tail() {
        /* now a points to b */
        *link.borrow_mut() = Rc::clone(&b);
    }

    /*
      - the recursion happens when println!
        try to print the next item in full
      - so since we won't be able to print
        the next item in full now as there
        is a reference cycle so it will
        overflow the stack
    */

    //println!("a next item = {:?}", a.tail());
}
