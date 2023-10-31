use std::io;
use validator::validate_email;

fn password_checker() {
    let mut password_length = String::new();

    println!("hello, welcome to the password generator!");

    println!("how long do you want the password to be?  ");

    let stdin = io::stdin();
    stdin
        .read_line(&mut password_length)
        .expect("There is a problem at the input section!");

    let password_length: u32 = password_length
        .trim()
        .parse()
        .expect("Error occured in generation!");

    println!("we will generate the password in a bit...");

    println!("This is the length check: {}", password_length);
}

fn email_checker() {
    println!("Welcome to the email checker! this is a pretty cool tool");
    println!("Enter the password you want to give!");
    let mut email = String::new();

    let stdin = io::stdin();
    stdin
        .read_line(&mut email)
        .expect("The input value is wrong!");

    let validator = email.trim();

    if validate_email(validator) {
        println!("your email is correct!");
    } else {
        println!("your email is kinda sus man!");
    }
}

fn main() {
    // loop this with the loop thing and a input way to take multiple inputs
        println!("welcome to the email checker and password generator: ");
        email_checker();
        password_checker();
    
}
