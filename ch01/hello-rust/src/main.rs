use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    // let x = 0;
    let list = [1,2];
    println!("Hello, world!");
    println!("first: {},\nsecond: {}", list[0], list[1]);
    let tuple = ("bla", 10, 4.5);
    println!("1: {}, 2: {}, 3: {}", tuple.0, tuple.1, tuple.2);
    let a = 2;
    let b = 3;
    let mut sum = a + b;
    println!("sum: {}", sum);
    sum = sum +1;
    println!("sum: {}", sum);

    loop {
        let secret_number = rand::thread_rng().gen_range(1, 101);
        println!("secret: {}", secret_number);
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("Your guessed: {}", guess);
        let guess: u64 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "You big!".red()),
            Ordering::Equal => {
                println!("{}", "Too win!".green());
                break;
            },
        }
    }
}
