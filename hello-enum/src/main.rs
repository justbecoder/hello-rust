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
    println!("print coin -> {}", value_in_cents(Coin::Penny))
}
