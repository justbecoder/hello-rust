use crate::hello::test::hi;

// 同名方法使用as关键字重新赋予名称
// 使用{}来简写同路径mod
use crate::hello::test::test2::{back_of_house::fix_incorrect_order, hi as hi_from_test2};

// 告诉rust，在hello.rs中查找
pub mod hello;

fn main() {
    println!("Hello, world!");

    //
    hi();

    hi_from_test2();

    fix_incorrect_order();
}
