use std::io;

fn factorial_calc() -> Option<u64> {
    println!("Enter a number to calculate its factorial:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num = match input.trim().parse::<u64>() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a valid positive number.");
            return None;
        }
    };

    let result = (1..=num).product();
    println!("The factorial of {} is {}", num, result);
    Some(result)
}

fn main() {
    println!("Please choose a script: \n (1) Factorial calculator");

    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice).expect("Failed to read line");
    let n_choice = user_choice.trim().parse::<u8>().expect("Please enter a number");

    match n_choice {
        1 => {
            factorial_calc();
        }
        _ => {
            println!("Invalid choice.");
        }
    }

}
