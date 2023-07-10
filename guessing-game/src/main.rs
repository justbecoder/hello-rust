use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number");

    // 生产随机数
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is : {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    // {} 表示占位符，顺序一次对应
    // println!("this is a plus x = {},y = {}, x + y = {}", 1, 2, 1 + 2);
}
