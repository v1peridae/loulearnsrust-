use std::rc::Rc;
//15.1
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    //PT2
    #[derive(Debug)]
    #[allow(dead_code)]
    enum BoxList {
        Cons(i32, Box<BoxList>),
        Nil,
    }

    use BoxList::{Cons as BoxCons, Nil as BoxNil};
    let list = BoxCons(1, Box::new(BoxCons(2, Box::new(BoxCons(3, Box::new(BoxNil))))));
    println!("list = {:?}", list);

    #[derive(Debug)]
    #[allow(dead_code)]
    enum RcList {
        Cons(i32, Rc<RcList>),
        Nil,
    }

    use RcList::{Cons as RcCons, Nil as RcNil};

    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    let b = RcCons(3, Rc::clone(&a));
    let c = RcCons(4, Rc::clone(&a));

    println!("a = {:?}, b = {:?}, c = {:?}", a, b, c);

    //PT 3
    #[derive(Debug)]
    #[allow(dead_code)]
    struct BigAhhStruct {
        bas: [u8; 10000],
    }

    let big_struct = BigAhhStruct {
        bas: [0; 10000],
    };

    let big_struct_ptr = Box::new(big_struct);
    println!("big_struct_ptr = {:?}", big_struct_ptr);


    //15.2
    let x = Rc::new(5);
    let y = Rc::clone(&x);
    let z = Rc::clone(&x);
    println!("x = {}, y = {}, z = {}", x, y, z);
}

