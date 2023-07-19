use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input yout Guess.");
        //创建一个可变变量，该变量绑定到一个新的字符串实例
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("The secret number is: {secret_number}");
        println!("You guessed: {guess}");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
