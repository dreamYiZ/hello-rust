enum List {
    Cons(i32, Rc<List>),
    Nil,
}


use crate::List::{Cons, Nil};

use std::rc::Rc;

fn main() {

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    // let b = Cons (3, Box::new(a));
    // let c = Cons(4, Box::new (a));
    let b:List = Cons(3, Rc::clone(&a));

    println!("count is {}", Rc::strong_count(&a));
    {
        let c:List = Cons(5, Rc::clone(&a));
        println!("count is {}", Rc::strong_count(&a));

    }
    println!("count is {}", Rc::strong_count(&a));

}
