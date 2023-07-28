// 引入map
use std::collections::HashMap;

pub mod exam;

fn main() {
    println!("Hello, world!");

    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    let team_name = String::from("yellowa");

    let score: i32 = scores.get(&team_name).copied().unwrap_or(0);

    println!("score is {score}");

    let mut s1 = String::from("abc");

    let s2: &String = &s1;

    // s1.push_str("def");

    println!("s1 is {s1}");
    println!("s2 is {s2}");

    let s3 = &s1[1..2];
    println!("s3 is {s3}");

    for s in s1.chars() {
        println!("current char is {s}");
    }

    for (key, value) in &scores {
        println!("key = {}, value={}", key, value);
    }

    let blue = String::from("blue");
    scores.insert(blue.to_string(), 100);
    scores.insert(String::from("red"), 100);

    // 在没有orange时进行插入
    scores.entry(String::from("orange")).or_insert(500);
    // red已经存在，不会重新插入
    scores.entry(String::from("red")).or_insert(500);
    println!("blue is {}", blue);

    println!("scores is {:?}", scores);

    let mut text_map = HashMap::new();
    let text = "Hello world wonderful world";

    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);

        // 使用*进行解引用
        *count += 1;
        println!("count == {}", count);
    }

    let mut nums = vec![1, 2, 3, 4, 5, 5, 5, 5, 6, 7, 8, 9, 9, 9, 9];

    // 2.5
    println!("5 / 2 , {}", 5.0 / 2.0);

    // 2
    println!("5 / 2 , {}", 5 / 2);

    let median = exam::find_median(&mut nums);

    let modes = exam::find_mode(&nums);

    println!("median === {}", median);
    println!("modes === {:?}", modes);
}
