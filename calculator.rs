
//needs to be renamed to main.rs
use std::io;
fn input(user_message: &str) -> io::Result<String> {
    use std::io::Write;

    print!("{}", user_message);

    // flush forces the print statement to the display.
    io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim_end().to_owned())
}
fn main () {
    let num1 = input("Input number: ")
        .unwrap()
        .parse::<i32>()
        .expect("Expected a number");
    let operator = input("Input operator +, -, /, *: ").unwrap();
    let num2 = input("Input number: ")
        .unwrap()
        .parse::<i32>()
        .expect("Expected a number");

    if operator == "+" {
        let total: i32 = num1 + num2;
        println!("{} {} {} = {}", num1, operator, num2, total);
    }
    if operator == "-" {
        let total: i32 = num1 - num2;
        println!("{} {} {} = {}", num1, operator, num2, total);
    }
    if operator == "/" {
        let total: i32 = num1 / num2;
        println!("{} {} {} = {}", num1, operator, num2, total);
    }
    if operator == "*" {
        let total: i32 = num1 * num2;
        println!("{} {} {} = {}", num1, operator, num2, total);
    }
}
