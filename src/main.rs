use validator::validate_email;
use std::io;



fn main() {
    println!("Welcome to the email checker! this is a pretty cool tool");
    let mut email = String::new();

    let stdin = io::stdin();
    stdin.read_line(&mut email)
    .expect("The input value is wrong!");

    let validator = email.trim();

    if validate_email(validator) {
        println!("your email is correct!");
    } else {
        println!("your email is kinda sus man!");
    }
}
