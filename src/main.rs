// use rand::seq::SliceRandom;
use rand::Rng;
use std::io;
use validator::validate_email;

fn password_checker() {
    // initializing contents of pass
    let mut password_length = String::new();

    let alphabets: [&str; 26] = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];
    let mut final_password: Vec<&str> = Vec::new();

    println!("hello, welcome to the password generator!");

    println!("how long do you want the password to be?  ");

    // reading user input
    let stdin = io::stdin();
    stdin
        .read_line(&mut password_length)
        .expect("There is a problem at the input section!");

    let mut password_lengths: u32 = password_length
        .trim()
        .parse()
        .expect("Error occured in generation!");

    let mut password_length: u32 = password_lengths + 1;

    println!("we will generate the password in a bit...");

    // for loop to process everything

    for n in 1..password_length {
        // random number finder and placing in index to get the gucci shit
        let mut rng = rand::thread_rng();
        let random_index: usize = rng.gen_range(0..password_length as usize);
        let mut letter = &alphabets[random_index];

        final_password.push(*letter);
    }

    // printing of the number
    println!("final password {}", final_password.join(""));

    // println!("This is the length check: {}", password_length);
}

fn email_checker() {
    // pretty straight forward and self explanatory
    // by that i mean this is mainly from the validator crate
    println!("Welcome to the email checker! this is a pretty cool tool");
    println!("Enter the email you want to give!");
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
