use std::io::{self, Write};

fn main() {
    println!("✧･ﾟ: *✧･ﾟ:* Your Task Manager *:･ﾟ✧*:･ﾟ✧");

    let mut tasks: Vec<String> = Vec::new();
    loop {
        println!("What would you wanna do?");
        println!("1. Add a task");
        println!("2. Complete a task");
        println!("3. View all tasks");
        println!("4. Remove a task");
        println!("5. Exit");

        print!("Enter a number to pick: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please enter a valid number");

        match choice {
            1 => {
                print!("Enter your task: ");
                io::stdout().flush().unwrap();
                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("Failed to read line");
                tasks.push(task.trim().to_string());
                println!("Task added successfully!");
            }
            2 => {
                if tasks.is_empty() {
                    println!("No tasks yet! Time to add some!");
                } else {
                    println!("\nYour Task List:");
                    for (index, task) in tasks.iter().enumerate() {
                        println!("{}: {}", index + 1, task);
                    }
                    print!("Enter the task number to complete: ");
                    io::stdout().flush().unwrap();
                    
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    if let Ok(index) = input.trim().parse::<usize>() {
                        if index > 0 && index <= tasks.len() {
                            tasks.remove(index - 1);
                            println!("Task completed successfully!");
                        } else {
                            println!("Invalid task number!");
                        }
                    }
                }
            }
            3 => {
                if tasks.is_empty() {
                    println!("No tasks yet! Time to add some!");
                } else {
                    println!("\nYour Task List:");
                    for (index, task) in tasks.iter().enumerate() {
                        println!("{}: {}", index + 1, task);
                    }
                }
            }
            4 => {
                if tasks.is_empty() {
                    println!("No tasks yet! Time to add some!");
                } else {
                    println!("\nYour Task List:");
                    for (index, task) in tasks.iter().enumerate() {
                        println!("{}: {}", index + 1, task);
                    }
                    print!("Enter the task number to remove: ");
                    io::stdout().flush().unwrap();
                    
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    if let Ok(index) = input.trim().parse::<usize>() {
                        if index > 0 && index <= tasks.len() {
                            tasks.remove(index - 1);
                            println!("Task removed successfully!");
                        } else {
                            println!("Invalid task number!");
                        }
                    }
                }
            }
            5 => {
                println!("Bye! ✧･ﾟ: *✧･ﾟ:*");
                break;
            }
            _ => println!("Invalid choice! Please enter a number between 1 and 5"),
        }
    }
}
