use std::io;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {
    println!("Simple Calculator\n");
    println!("Select\n 1 - Add\n 2 - Subtract\n 3 - Multiply\n 4 - Divide");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let number: i32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number entered. Defaulting to 1.");
            1
        }
    };

    let mut first_choice = String::new();
    println!("Enter the first number: ");
    io::stdin()
        .read_line(&mut first_choice)
        .expect("Failed to read line");

    let first_number: i32 = match first_choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number entered. Defaulting to 1.");
            1
        }
    };

    let mut second_choice = String::new();
    println!("Enter the second number: ");
    io::stdin()
        .read_line(&mut second_choice)
        .expect("Failed to read line");

    let second_number: i32 = match second_choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number entered. Defaulting to 1.");
            1
        }
    };

    let result = match number {
        1 => add(first_number, second_number),
        2 => subtract(first_number, second_number),
        3 => multiply(first_number, second_number),
        4 => divide(first_number, second_number),
        _ => {
            println!("Invalid number. Please select a number 1-4.");
            return;

        }

    };

    println!("Result: {}", result);
}
