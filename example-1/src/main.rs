use std::io;

fn main() {
    const FREEZING_TEMP: i32 = 32;
    const CONVERSION_MULTIPLIER: f32 = 5.0 / 9.0;
    let mut temperatures = Vec::new();

    loop {
        println!("Enter the temperature in Fahrenheit or 'q' to quit ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

         let input = input.trim();
        if input.to_lowercase() == "q" {
            break;
        }
    
        match input.parse::<f32>() {
            Ok(temp_f) => {
                let temp_c = (temp_f - FREEZING_TEMP as f32) * CONVERSION_MULTIPLIER;
                temperatures.push((temp_f, temp_c));
                println!("{}F is {:.1}C", temp_f, temp_c);
            }
            Err(_) => {
                println!("Enter a valid number or 'q' to quit");
                continue;
            }
        }
    }

    println!("Here are all the temperatures you entered: ");
    for (f, c) in &temperatures {
        println!("{}F = {:.1}C", f, c);
    }
}