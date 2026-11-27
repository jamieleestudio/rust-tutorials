// modules crate 的可执行入口
// 这个 crate 同时有 lib（src/lib.rs）和 bin（src/main.rs）两个 target：
// main.rs 通过 crate 名 `modules` 引用 lib 的公开内容，对标"应用入口调用工具包"

use modules::customer::eat_at_restaurant;
use modules::hosting; // 用的是 lib.rs 重导出（pub use）的路径
// use modules::front_of_house::hosting; // 原始路径同样可用

fn main() {
    // 完整路径写法：crate::module::submodule::function
    modules::front_of_house::hosting::add_to_waitlist();

    // use 引入后的短路径（对标 import + 短类名）
    hosting::add_to_waitlist();

    eat_at_restaurant();
}
