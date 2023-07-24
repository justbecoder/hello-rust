#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {
        // 方法体
        println!("what is self ? === {:?}", &self)
    }
}
// #[derive(Debug)]
// enum Option<T> {
//     None,
//     Some(T),
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Luck! Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four === {:?}, six ==== {:?}", four, six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home === {:?}", home);
    println!("loopback === {:?}", loopback);

    // 可以将任意类型的数据放入枚举成员中：例如字符串、数字类型和结构体，甚至可以包含另一个枚举

    let m = Message::Write(String::from("abc"));

    m.call();

    // Option
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // match
    println!("print coin -> {}", value_in_cents(Coin::Penny));

    //
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("print six -> {:?}", six);
    println!("print None -> {:?}", none);

    let dice_roll = 9;

    // 匹配必须是穷尽的
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // 使用other接收了匹配到的值
        // other => move_player(other),
        // 使用_表示匹配，但是不接收值
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {
        println!("this is num_spaces {}", num_spaces);
    }

    fn reroll() {
        println!("please roll again");
    }

    let a = 10;
    let b = 10;

    if a == b {
        println!("a equals b");
    }

    let config_max = Some(122u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    let coin = Coin::Penny;
    if let Coin::Quarter = coin {
        println!("equals Quarter");
    } else {
        count += 1;
    }

    println!("now coin is {}", count)
}
