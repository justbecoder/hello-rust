// 使用fn关键字声明函数，main函数时入口函数
fn main() {
    println!("Hello, world!");

    // 函数调用
    another_fun(10, "xadfdsf");

    let five = return_five();
    println!("返回值five={five}");
}

// 1. 函数名规范，所有字母都使用小写，同时使用下划线分割单词
// 2. 函数签名中，必须声明每个参数的类型，多个参数使用逗号分割
fn another_fun(x: i32, label: &str) {
    println!("function param x:{}，label: {}", x, label);

    let y = {
        let x = 10;
        // x + 1; // 注意：这个位置使用分号结尾，表明是一个语句，语句没有返回值；
        x + 1 // 注意：该位置没有分号，表明是一个表达式，表达式有返回值；
    };

    println!("现在y的值 {}", y);
}

// 返回值
fn return_five() -> i32 {
    // 这是一个表达式，返回的是一个值
    // 5

    // 使用return关键字，返回指定的值，分号结尾，语句
    return 5;
}
