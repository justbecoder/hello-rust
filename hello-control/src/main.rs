fn main() {
    println!("Hello, control flow!");

    // control flow

    let x = 10;

    // 在if条件中，必须是一个布尔值，不存在类型转换问题
    if x == 10 {
        println!("x value === 10");
    }

    // loop循环
    let mut count = 0;

    // 使用result接受loop循环返回值
    let result = loop {
        count = count + 1;

        if count == 10 {
            break count * 2;
        }
    };

    println!(
        "loop completed! count's value {} result == {}",
        count, result
    );

    // 使用循环标签，搭配break/continue使用
    let mut c = 0;

    'count_c_up: loop {
        println!("count->c = {c}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if c == 2 {
                break 'count_c_up;
            }

            remaining -= 1;
        }

        c += 1;
    }

    println!("End count->c = {c}");

    // while循环
    let mut number = 3;

    while number != 0 {
        println!("current number === {number}");

        number -= 1;
    }

    println!("while end! LEFTOFF!");

    // 使用for遍历集合
    let a = [10, 20, 30, 40, 50];

    for ele in a {
        println!("current value === {ele}");
    }

    for number in (1..5).rev() {
        println!("{number}!");
    }

    println!("LEFTOFF!");

    fi(10);
}

fn fi(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 1;
    }

    let mut x = 1;
    let mut y = 1;
    let mut sum = 0;
    let mut i = 3;
    while i <= n {
        sum = x + y;
        x = y;
        y = sum;
        println!("current x: {x} y: {y} sum: {sum}");
        i += 1;
    }

    println!("第{n}项的值是{sum}");

    return sum;
}
