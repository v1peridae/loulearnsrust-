fn main() { 
    let x = 5;
    println!("Immutable x: {}", x);
    
    // Mutable variable
    let mut y = 10;
    println!("Original y: {}", y);
    y = 15;
    println!("Modified y: {}", y);
    
    // Constants - must have type and are always immutable
    const MAX_POINTS: u32 = 100_000;
    println!("Constant: {}", MAX_POINTS);
    
    // Shadowing - creating a new variable with the same name
    let spaces = "   "; // string type
    let spaces = spaces.len(); // number type
    println!("Shadowed variable: {}", spaces);
    
    // Types
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'a';
    
    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character);
    
    let tuple: (i32, f64, char) = (500, 6.4, 'z');
    println!("Tuple: {:?}", tuple);  // Prints whole tuple
    
    // Accessing tuple elements
    let (x, y, z) = tuple; 
    println!("Tuple values: {}, {}, {}", x, y, z);
    println!("First value: {}", tuple.0);  // Direct access
    
    // Array w inferred fixed length and same type
    let array = [1, 2, 3, 4, 5];  
    println!("Array: {:?}", array);
    
    // Array w explicit type and length
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June",
                             "July", "August", "September", "October", "November", "December"];
    println!("Third month: {}", months[2]);  
    
    // Array w repeated values
    let repeated = [3; 5]; 
    println!("Repeated array: {:?}", repeated);
}
