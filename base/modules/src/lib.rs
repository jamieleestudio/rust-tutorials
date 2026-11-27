//! 模块系统（crate 根：src/lib.rs）
//!
//! Rust 的模块层级：crate（对标整个 jar/工程） > package（对标 Java package） > module
//! 本 crate 演示：内联模块、独立文件模块、可见性、重导出

// ============ 1. 内联模块：对标同一个 .java 文件里的嵌套类/包结构 ============
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("客人已加入等位名单");
        }

        // pub(crate)：仅 crate 内可见，对标 package-private
        pub(crate) fn notify_waitlist() {
            println!("（内部）通知等位队列");
        }
    }
}

// ============ 2. 私有模块 + 访问修饰符 ============
// 没写 pub 的模块只对本 crate 可见，对标 package-private 的包
#[allow(dead_code)] // 教学示例：部分成员只在测试中构造
mod back_of_house {
    // struct 的字段默认私有！对标 Java 字段 private + getter/setter
    pub struct Breakfast {
        pub toast: String,       // pub 字段对外可访问（对标 public 字段）
        seasonal_fruit: String,  // 私有字段，外部无法直接构造含私有字段的 struct
    }

    impl Breakfast {
        // 对外提供工厂方法，对标构造器
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: toast.to_string(),
                seasonal_fruit: "桃子".to_string(),
            }
        }

        // getter（Rust 惯例不写 get 前缀）
        pub fn fruit(&self) -> &str {
            &self.seasonal_fruit
        }
    }

    // 枚举变体自动继承枚举的可见性（struct 则每个字段单独控制）
    pub enum Dessert {
        Cake, // 自动 pub，无需逐个标注
        Pie,
    }
}

// ============ 3. 独立文件模块：src/customer.rs ============
// 对标 com.example.customer 包（customer.rs 就是一个包）
pub mod customer;

// ============ 4. 重导出：打造对外 API 门面 ============
// 对标：把内部实现包装成稳定的公共 API
pub use front_of_house::hosting; // 现在外部可以直接 use modules::hosting

// crate 内部随便用私有模块（对标同包访问）
#[allow(dead_code)]
fn internal_call() {
    let _cake = back_of_house::Dessert::Cake;
    front_of_house::hosting::notify_waitlist(); // crate 内可见
}

#[cfg(test)]
mod tests {
    use super::back_of_house::Breakfast;

    #[test]
    fn private_field_needs_constructor() {
        // let b = Breakfast { toast: ... , seasonal_fruit: ... }; // ❌ 私有字段不能外部构造
        let b = Breakfast::summer("全麦");
        assert_eq!(b.toast, "全麦"); // pub 字段可读
        assert_eq!(b.fruit(), "桃子"); // 私有字段只能走 getter
    }

    #[test]
    fn reexport_works() {
        // super::hosting 就是重导出的 front_of_house::hosting
        super::hosting::add_to_waitlist();
    }
}
