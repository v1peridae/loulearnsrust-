use std::io;

fn main() {
// if-else statement
let mut input = String::new();
println!("Enter a number");
io::stdin().read_line(&mut input).expect("Failed to read line");
let x: i32 = input.trim().parse().expect("Please enter a valid number");
if x == 5{
    println!("x is 5");
} else {
    println!("x is not 5");
}

//match statement
let x = 5;
match x {
    1 => println!("x is 1"),
    2 => println!("x is 2"),
    _ => {},
}

//while loop
let mut x = 0;
while x < 10{
    println!("x is {}", x);
    x += 1;
}

//loop
let mut x = 10;
loop{
    println!("x is {}", x);
    x += 1;
    if x == 21{
        break;
    }
}

//for loop
for x in 21..31{
    println!("x is {}", x);
}
}