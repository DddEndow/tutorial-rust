use List::{Cons, Nil};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    // let list = Cons(1,
    //     Box::new(Cons(2,
    //         Box::new(Cons(3,
    //             Box::new(Nil))))));
    //
    // let x = 5;
    // // let y = &x;
    // // let y = Box::new(x);
    // let y = MyBox::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointers dropped before the end of main.");

    // let rc_a = Rc::new(Cons(5,
    //     Rc::new(Cons(10,
    //         Rc::new(Nil)))));
    // println!("count after creating a = {}", Rc::strong_count(&rc_a));
    // let rc_b = Cons(3, Rc::clone(&rc_a));
    // println!("count after creating b = {}", Rc::strong_count(&rc_a));
    // {
    //     let rc_c = Cons(4, Rc::clone(&rc_a));
    //     println!("count after creating c = {}", Rc::strong_count(&rc_a));
    // }
    // println!("count after c goees out of scope = {}", Rc::strong_count(&rc_a));

    // let ref_a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    //
    // println!("a initial rc count = {}", Rc::strong_count(&ref_a));
    // println!("a next item = {:?}", ref_a.tail());
    //
    // let ref_b = Rc::new(Cons(10, RefCell::new(Rc::clone(&ref_a))));
    //
    // println!("a rc count after b creation = {}", Rc::strong_count(&ref_a));
    // println!("b initial rc count = {}", Rc::strong_count(&ref_b));
    // println!("b next item = {:?}", ref_b.tail());
    //
    // if let Some(link) = ref_a.tail() {
    //     *link.borrow_mut() = Rc::clone(&ref_b);
    // }
    //
    // println!("b rc count after changing a = {}", Rc::strong_count(&ref_b));
    // println!("a rc count after changing a = {}", Rc::strong_count(&ref_a));
    //
    // // println!("a next item = {:?}", ref_a.tail());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
