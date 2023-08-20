use rand::Rng;
use rand::seq::SliceRandom;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let spec = "_.*/";
    let alph_lower = "abcdefghijklmnopqrstuvwxyz";
    let alph_upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut rng = rand::thread_rng();

    println!("Input password length: ");
    let mut length = String::new();
    std::io::stdin().read_line(&mut length).expect("Failed to read line");
    let length: usize = length.trim().parse().expect("Please enter a number");

    let mut password = String::new();
    for _ in 0..length {
        let r = rng.gen_range(0..3);
        if r == 0 {
            password.push(*spec.chars().collect::<Vec<_>>().choose(&mut rng).unwrap());
        } else if r == 1 {
            password.push_str(&rng.gen_range(0..10).to_string());
        } else {
            let choices = [alph_lower, alph_upper];
            let choice = choices.choose(&mut rng).unwrap();
            password.push(*choice.chars().collect::<Vec<_>>().choose(&mut rng).unwrap());
        }
    }

    println!("Your new password is: {}", password);

    let mut file = OpenOptions::new().append(true).create(true).open("pass.txt")?;
    writeln!(file, "{}", password)?;

    println!("Your password has been written to the file");

    Ok(())
}