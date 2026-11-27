// 第一个 Rust 程序
// 对标 Java：public class Main { public static void main(String[] args) }

fn main() {
    // println! 是宏（macro），不是函数，感叹号是宏调用的标志
    // 对标 Java 的 System.out.println
    println!("Hello, Rust!");

    // 格式化占位符 {} （对标 String.format / MessageFormat）
    let name = "Rust";
    let edition = 2021;
    println!("你好, {}! 当前 edition {}", name, edition);

    // 内联捕获：直接写变量名（Rust 1.58+），对标 Java 15+ 的 formatted
    let x = 42;
    println!("x = {x}");

    // {:?} 调试格式化（对标 Lombok @Data 生成的 toString）
    // 元组没有实现 Display，只能用 {:?}
    let point = (3, 5);
    println!("point = {:?}", point);

    hello("Rust 学习者");
}

/// 函数定义：fn + 参数名: 类型 + 返回类型 -> T
/// 对标 Java：static void hello(String name)
/// 文档注释 /// 会生成文档（对标 Javadoc）
fn hello(name: &str) {
    // &str 是字符串切片（只读借用），类似 Java String 的不可变引用
    println!("欢迎 {name} 来到 Rust 世界!");
}

// 单元测试直接写在源码文件里，语言内建（对标 JUnit 5）
// cargo test 即可运行，无需任何测试依赖
#[cfg(test)]
mod tests {
    #[test]
    fn test_hello() {
        // assert_eq! 对标 JUnit 的 assertEquals / AssertJ 的 isEqualTo
        assert_eq!(1 + 1, 2);
        assert_ne!(1, 2);
        assert!(true, "条件断言，对标 assertTrue");
    }
}
