// 函数
// 对标 Java：方法定义、重载、默认参数、可变参数

fn main() {
    // 1. 基本调用
    println!("add = {}", add(1, 2));

    // 2. if 表达式替代三元运算符
    println!("max = {}", max_or_default(3, 9));

    // 3. Option 替代 null 返回值
    let nums = vec![1, 3, 5, 8, 9];
    match find_first_even(&nums) {
        Some(v) => println!("第一个偶数: {v}"),
        None => println!("没有偶数"),
    }
    // 对标 Java：nums.stream().filter(n -> n % 2 == 0).findFirst().orElse(null)

    // 4. 用 Option 模拟默认参数（Rust 没有默认参数，也没有重载！）
    println!("{}", greet("小明", None));
    println!("{}", greet("小红", Some("早上好")));
    // Java 可以 greet(name) / greet(name, greeting) 重载实现同样效果

    // 5. 语句 vs 表达式 —— Rust 的核心差异
    //    块 { } 的值是最后一个表达式的值（无分号）
    let y = {
        let a = 1;
        let b = 2;
        a + b // 注意：没有分号！加分号就变成语句，返回 ()
    };
    println!("块表达式的值 = {y}");

    // 6. 提前返回用 return（对标 Java return，正常路径省略 return 更地道）
    println!("{}", classify(15));
    println!("{}", classify(0));
    println!("{}", classify(-5));

    // 7. 可变参数用切片模拟（对标 Java 的 String... args）
    println!("{}", join_all(&["a", "b", "c"]));

    // 8. 函数也是值：可以赋给变量、当参数传（对标 Java 方法引用 Main::add）
    let f: fn(i32, i32) -> i32 = add;
    println!("通过函数指针调用 = {}", f(10, 20));
}

/// 基本函数：fn 函数名(参数: 类型) -> 返回类型
/// 对标 Java：static int add(int a, int b)
fn add(a: i32, b: i32) -> i32 {
    a + b // 最后一个表达式就是返回值，不写 return、不加分号
}

/// 对标 Java 三元运算符：a > b ? a : b
fn max_or_default(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

/// 返回 Option 而不是 null（十亿美元错误的解法）
/// 对标 Java：Optional<Integer>
fn find_first_even(nums: &[i32]) -> Option<i32> {
    nums.iter().find(|&&n| n % 2 == 0).copied()
}

/// Rust 没有方法重载！用 Option 参数模拟默认值
/// 对标 Java 的两个重载 greet(String) / greet(String, String)
fn greet(name: &str, greeting: Option<&str>) -> String {
    // unwrap_or：有值用值，没值用默认 —— 对标 Optional.orElse
    let greeting = greeting.unwrap_or("你好");
    format!("{greeting}, {name}!")
}

/// 提前返回：用 return 关键字 + 分号
fn classify(n: i32) -> &'static str {
    if n > 0 {
        return "正数"; // 提前返回必须带 return 和分号
    }
    if n == 0 {
        "零" // 最后一行作为表达式返回
    } else {
        "负数"
    }
}

/// 切片参数模拟可变参数：对标 Java 的 void join(String... parts)
fn join_all(parts: &[&str]) -> String {
    parts.join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expression_is_return_value() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn option_instead_of_null() {
        assert_eq!(find_first_even(&[1, 3, 4]), Some(4));
        assert_eq!(find_first_even(&[1, 3, 5]), None);
    }

    #[test]
    fn default_param_via_option() {
        assert_eq!(greet("张三", None), "你好, 张三!");
        assert_eq!(greet("张三", Some("嗨")), "嗨, 张三!");
    }

    #[test]
    fn varargs_via_slice() {
        assert_eq!(join_all(&["x", "y"]), "x-y");
        assert_eq!(join_all(&[]), "");
    }

    #[test]
    fn early_return() {
        assert_eq!(classify(7), "正数");
        assert_eq!(classify(0), "零");
        assert_eq!(classify(-7), "负数");
    }
}
