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

    no_dangle();

    let len = first_word(&String::from("value"));
    println!("字符串长度: {}", len);

    let len_a = first_word(&"a bc sdfsdf".to_string());
    println!("字符串长度: {}", len_a);

    str_slice();
}

fn print_str<T: Display>(str: T) {
    println!("print str: {}", str);
}

// 注意避免出现悬垂指针问题
fn no_dangle() -> String {
    // let s = "abc";
    // return s.to_string();

    let s = String::from("abcs");
    return s;
}

fn first_word(s: &String) -> usize {
    // 转为字节数组
    let bytes = s.as_bytes();

    // 使用iter创建迭代器
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}

fn str_slice() {
    let s = String::from("Hello World!");

    let hello = &s[0..5];
    let world = &s[6..12];
    let hello_no_zero = &s[..5];
    let len: usize = s.len();
    let world_no_end = &s[..len];

    println!("Hello ---- {}", hello);
    println!("World! --- {}", world);
    println!("hello_no_zero! --- {}", hello_no_zero);
    println!("world_no_end! --- {}", world_no_end);

    println!("输出单词 {}", first_word_str(&s));

    let s_str = "ab c";
    println!("first world: {}", first_word_str(s_str));

    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[1..3];
    assert_eq!(
        a_slice,
        &[2, 3],
        "we are testing addition with {:?} and {:?}",
        a_slice,
        &[2, 3]
    );
}

fn first_word_str(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}
