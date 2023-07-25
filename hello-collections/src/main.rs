fn main() {
    println!("Hello, world!");

    // 初始化vec，存储数据类型i32
    // let mut v: Vec<i32> = Vec::new();

    // 使用vec!宏，创建一个新的vector，同时可以指定初始值
    let mut v = vec![1, 2, 3];

    // 放入数据 - 更新vec
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    // 读取元素
    // let third = &v[200]; // 当使用[]访问一个不存在的index时会Panic!
    // let third = &v.get(2000); // 使用get访问时，访问一个不存在的index时，返回一个None
    let third = &v[2];

    println!("the third element is {:?}", third);

    // 遍历vector元素
    for i in &v {
        println!("{i}");
    }

    // 遍历并改变
    for i in &mut v {
        // * 解引用运算符
        *i += 10;
    }

    for i in &v {
        println!("{i}");
    }
}
