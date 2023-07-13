use std::fmt::Display;

fn main() {
    let s1 = String::from("Hello");
    println!("Hello, world! {}", s1);
    let s2 = s1;
    let s3 = s2.clone();
    // println!("Hello, world! {}", s1); // 此处为error
    println!("Hello, world! {}", s2);
    println!("Hello, world! {}", s3);

    let x = 10;
    let y = x;

    println!("x value == {x}");

    // tuple
    let tup = (1, 2, 3, "abc");

    // 当出现了String::from之后，这个tup就不能在被赋值到tup1之后再被使用
    // let tup = (1, 2, 3, "abc", String::from("demo"));
    let tup1 = tup;

    println!("look tuple: {:?}", tup);
    let s = "anc";
    print_str(s);
    println!("这是还能输出s吗? --- {}", s);

    let sA = String::from("abc");
    print_str(sA);
    // 注意变量sA被移动到函数print_str中，此处sA不在有效
    // println!("这是还能输出s吗? --- {}", sA); // Error

    let sB = String::from("value");

    // 注意此处传递值的方式是引用
    print_str(&sB);

    println!("这里还能输出sB吗? --- {}", sB);
}

fn print_str<T: Display>(str: T) {
    println!("print str: {}", str);
}
