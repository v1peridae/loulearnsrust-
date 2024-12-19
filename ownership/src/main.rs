fn main() {
    // Basic ownership and moving ownership
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // println!("{}", s1); // Error - s1 is no longer valid
    println!("s2: {}", s2);

    // Clone for deep copy
    let s3 = String::from("world");
    let s4 = s3.clone(); // Creates a copy
    println!("s3: {}, s4: {}", s3, s4); 

    // Ownership and functions
    let s5 = String::from("ownership");
    takes_ownership(s5);
    // println!("{}", s5); // Error - s5 was moved

    // References and borrowing
    let s6 = String::from("borrow");
    borrow_string(&s6); // Passed as a reference
    println!("Can still use s6: {}", s6);

    //Mutable references
    let mut s7 = String::from("mut");
    change_string(&mut s7);
    println!("Changed string: {}", s7);

    // Multiple references
    let s8 = String::from("multiple");
    let r1 = &s8;
    let r2 = &s8;
    println!("Multiple references: {} and {}", r1, r2);
}
