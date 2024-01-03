enum List {
    Cons(i32, Box<List>),
    Nil
}

enum ListRef<'a> {
    Cons(i32, &'a ListRef<'a>),
    Nil
}

use crate::List:: {Cons, Nil};
fn main() {

    // let b = Box::new(5);

    // println!("b = {}", b);

    let x = 5;
    // let y = &x;

    let y = Box::new(x);

    


     println!("{:p}, {:p}, {}", &x, y, x==*y);


     println!("{}", Some("ok") == Some("ok"));

    // let list = Cons(1, Cons(2, Cons(3, Nil))); //throw exception, 

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));


    // println!("Hello, world!");
}
