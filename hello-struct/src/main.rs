struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法 - 传递参数
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数，不同于方法，调用时使用::
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn build_user(email: String, username: String) -> User {
    return User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    };
}

fn area(width: i32, height: i32) -> i32 {
    width * height
}

fn area_rect(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    println!("Hello, world! --- Struct");

    let user1: User = User {
        active: true,
        username: String::from("huxiaoshuai"),
        email: String::from("huxiaoshuai@gmail.com"),
        sign_in_count: 1,
    };

    // println!("hello -- user: {:?}", user1);

    let user2: User = build_user(String::from("demo@gmail.com"), String::from("huxiaoshuai"));

    let user3 = User {
        active: user2.active,
        username: user2.username,
        email: String::from("another@gmail.com"),
        sign_in_count: user2.sign_in_count,
    };

    let mut user4 = User {
        email: String::from("user4@gmail.com"),
        ..user1
    };

    user4.username = String::from("value");

    // println!("打印user1的值 {}", user1.username); // error
    println!("打印user4的值 {}, email == {}", user4.username, user4.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}", origin);

    let subject = AlwaysEqual;

    println!("{:?}", subject);

    let width = 10;
    let height = 20;

    println!("area is {}", area(width, height));

    let rect1 = Rectangle {
        height: 20,
        width: 30,
    };

    dbg!("打印信息,;;; {}", &rect1);

    println!(
        "The area of the rectangle is {} square pixels",
        area_rect(&rect1)
    );

    println!("print rect1 === {}", rect1.width);
    println!("print rect1 === {}", &rect1.height);

    let a: &u32 = &rect1.height;

    println!("rect height {} --- a {} ", rect1.height, a);

    // rect2是Rectangle的实例
    let rect2 = Rectangle {
        width: 400,
        height: 200,
    };

    println!("area is {}", rect2.area());

    println!("rect2 can hod rect1 === {}", rect2.can_hold(&rect1));

    // 注意这个位置的调用方式，square是Rectangle的一个关联函数，不是一个方法
    println!("a square {:?}", Rectangle::square(29))
}
