fn deliver_order() {}

pub fn hi() {
    println!("I'm from test2");
    deliver_order();
}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();

        // 引用父级作用域关联函数
        super::deliver_order();
    }

    fn cook_order() {}
}
