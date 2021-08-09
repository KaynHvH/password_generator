use rand::Rng;
use std::io;
use std::num::ParseIntError;

pub fn add_letters() -> String {
    let mut rng = rand::thread_rng();
    let mut letters_len = String::new();
    let mut ret = String::new();

    println!("Write length of letters in password");

    io::stdin().read_line(&mut letters_len)
        .ok()
        .expect("failed to parse");


    for random_letters in 0..letters_len.trim().parse::<u32>().unwrap() {
        let letter: char = rng.gen_range('A'..='z');
        print!("{}", letter);
        ret.push(letter);
    }
    println!();
    ret
}

pub fn add_numbers() -> String {
    let mut rng = rand::thread_rng();
    let mut numbers_len = String::new();
    let mut ret = String::new();

    println!("Type length of numbers in password");

    io::stdin().read_line(&mut numbers_len)
        .ok()
        .expect("failed to parse");


    for random_numbers in 0..numbers_len.trim().parse::<u32>().unwrap() {
        let number: char = rng.gen_range('0'..='9');
        print!("{}", number);
        ret.push(number);
    }
    println!();
    ret
}

fn add_special_char() -> String {
    let mut rng = rand::thread_rng();
    let mut special_char_len = String::new();
    let mut ret = String::new();

    println!("Type length of special chars in password");

    io::stdin().read_line(&mut special_char_len)
        .ok()
        .expect("failed to parse");


    for random_special_chars in 0..special_char_len.trim().parse::<u32>().unwrap() {
        let special_char: char = rng.gen_range('!'..='#');
        print!("{}", special_char);
        ret.push(special_char);
    }
    println!();
    ret
}

fn main() {
    println!("Your password has generated!");
    println!("Password is below");
    println!("{}{}{}", add_letters(), add_numbers(), add_special_char());
}
