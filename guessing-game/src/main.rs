use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    // 生产随机数
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is : {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 对guess的类型检测
        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // 异常时继续执行
            Err(_) => {
                // println!("Please input valid number");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // {} 表示占位符，顺序一次对应
        // println!("this is a plus x = {},y = {}, x + y = {}", 1, 2, 1 + 2);

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
