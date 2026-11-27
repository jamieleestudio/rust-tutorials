//! 独立文件模块：customer
//! 文件位置 src/customer.rs，对标 com.example.customer 包

pub fn eat_at_restaurant() {
    // crate:: 从 crate 根开始写绝对路径（对标从包根写的全限定名）
    crate::front_of_house::hosting::add_to_waitlist();
    println!("顾客开始点餐...");
    // super:: 是相对路径，指向父模块（对标 ../）
    // super::front_of_house::hosting::notify_waitlist();
}
