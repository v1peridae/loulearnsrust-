fn main() {
    let result = divide(10, 2);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    
    match age("invalid") {
        Ok(age) => println!("Age: {}", age),
        Err(e) => println!("Error: {}", e),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn age(age: &str) -> Result<u8, String> {
    let age = age.parse::<u8>().map_err(|_| "Invalid age".to_string())?;
    if age > 150 {
        Err("Age out of range".to_string())
    } else {
        Ok(age)
    }}