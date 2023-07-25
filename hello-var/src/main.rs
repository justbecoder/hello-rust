const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("Hello, world!");

    let mut x = 5;

    println!("The value of x is: {x}");

    x = 10;

    println!("The value of x is: {x}");

    println!("{}", x);

    println!("The value of x constants: {THREE_HOURS_IN_SECONDS}");

    {
        // 隐藏赋值
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // 赋值tuple
    // 元组长度固定：一旦声明，长度不会增大或缩小
    let mut tup: (i32, i32, i32, &str) = (1, 2, 3, "abc");

    println!("{:?}", tup);

    println!("基于解构的形式0{}", tup.0);

    // array
    // 数组：每个元素类型都必须相同，同时数组长度固定
    let arr = [1, 2, 3, 4];

    println!("{:?}", arr);

    // 声明数组，同时设置初始值为3，长度为5
    let mut a = [3; 5];

    println!("{:?}，索引取值{}", a, a[0]);

    // 数组访问时使用[]，元组使用.
    a[0] = 10;
    tup.0 = 10;

    println!("{:?}，数组索引取值{}", a, a[0]);
    println!("{:?}，元组索引取值{}", tup, tup.0);

    let [b, c, s, r, t] = a;
}
