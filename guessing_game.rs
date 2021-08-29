//needs to be renamed to main.rs
use std::io;
use rand::Rng;




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
    let mut rng = rand::thread_rng();
    const MAXWRONG: u8 = 6;
    println!("Welcome to the guess the number game!!");
    let number: u8 = rng.gen_range(1..101);
    let mut tries: u8 = 1;
    let mut guess: u8 = input("Input number between 1-100: ")
        .unwrap()
        .parse::<u8>()
        .expect("Expected a number");

    while guess != number {
        //println!("{}", number);
        if guess > number {
            println!("Lower....");
        }
        else if guess < number {
            println!("Higher....")
        } else {
            println!("Correct");
            break;
        }
        tries += 1;
        guess = input("Input number between 1-100: ")
            .unwrap()
            .parse::<u8>()
            .expect("Expected a number");
    }
    if tries <= MAXWRONG {
        println!("You win!");
        println!("You had {} guesses and the max is {}", tries, MAXWRONG)
    } else {
        println!("To many guesses!!");
        println!("You had {} guesses but the max is {}", tries, MAXWRONG)
    }
}
