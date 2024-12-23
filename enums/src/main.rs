fn main() {
    enum Number{
        One = 1,
        Two = 2,
        Ten = 10,
    }

    println!("{}", Number::Two as i32);
    println!("{}", Number::Ten as i32);

    enum Example{
        Coordinates(f64, f64),
        Name(String),
    }

    let point = Example::Coordinates(1.0, 2.0);

    let name = Example::Name("Lou".to_string());


    if let Example::Coordinates(x, y) = point{
        println!("x: {}, y: {}", x, y);
    }

    if let Example::Name(ref name) = name {
        println!("Name: {}", name);
    }

    match name {
        Example::Name(ref name) => println!("My name is {}", name),
        _  => {}
    }

    // option enum
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;


    match x {
        Some(x) => {
            if x == 5 {
                println!("YOURE MISSING GO BACK TO ZERO");
            }
        }
        None => {
            println!("We don't know where you are, stay safe baddie");
        }
    }

}