// 字符串
// 对标 Java：String / StringBuilder
// Rust 有两种字符串：String（拥有所有权的）和 &str（借用的切片）—— 这是 Java 没有的二分法

fn main() {
    // ============ String vs &str ============
    // String：堆上、可增长、拥有所有权 —— 对标 StringBuilder
    let mut s = String::new();
    s.push_str("你好");
    s.push('，'); // 单个 char
    s.push_str("Rust");
    println!("String = {s}");

    // 字符串字面量就是 &str：对标 Java String 的不可变引用视图
    let literal: &str = "hello";
    println!("&str = {literal}");

    // &str 可以从 String 借出：对标 substring 的轻量视图（零拷贝）
    let view: &str = &s[3..7]; // 按字节切片！UTF-8 下 "好" 占 3 字节
    println!("切片视图 = {view}");

    // ============ 拼接：对标 + / StringBuilder / String.format ============
    let a = String::from("Hello");
    let b = String::from("Rust");
    // format!：对标 String.format，只借用不移动，最常用
    let d = format!("{a} again {b}");
    println!("format! 拼接 = {d}（a、b 仍然可用）");

    // + 运算符：左操作数被"吃掉"（所有权移动），Java 没有这种语义！
    let c = a + " " + &b; // a 在这里被移动进 +，之后不能再使用
    println!("+ 拼接 = {c}");
    // println!("{a}"); // ❌ 编译错误：value borrowed here after move —— 放开注释体验一下

    // ============ 常用方法速查 ============
    let s = "  Hello, Rust!  ";
    println!("trim       = '{}'", s.trim()); // 对标 strip()
    println!("upper      = {}", s.trim().to_uppercase()); // 对标 toUpperCase
    println!("contains   = {}", s.contains("Rust")); // 对标 contains
    println!("replace    = {}", s.trim().replace("Rust", "World"));
    println!("split      = {:?}", "a,b,c".split(',').collect::<Vec<_>>());
    println!("starts_with = {}", s.trim_start().starts_with("Hello"));

    // 字符串比较（对标 equals / equalsIgnoreCase）
    println!("eq ignore case = {}", "ABC".eq_ignore_ascii_case("abc"));

    // ============ 解析与转换 ============
    // parse：对标 Integer.parseInt，但返回 Result 而不是抛异常
    let n: i32 = "42".parse().expect("解析失败");
    println!("parse = {n}");
    let bad: Result<i32, _> = "abc".parse();
    println!("parse 失败 = {bad:?}"); // Err(ParseIntError...)，不抛异常

    // to_string：任何实现 Display 的类型都能转 String（对标 String.valueOf）
    let num_str = 3.14.to_string();
    println!("to_string = {num_str}");

    // ============ UTF-8：与 Java 最大的差异点 ============
    // Java String 内部是 UTF-16（char 是 2 字节代码单元）
    // Rust String 是 UTF-8（1~4 字节变长），所以：
    //   - 不能用下标直接访问第 N 个"字符"（O(1) 索引不存在）
    //   - 按字节切片可能 panic（切在字符中间）
    let cn = "中文abc";
    println!("字节长度 = {}，字符数 = {}", cn.len(), cn.chars().count());
    // cn[0] // ❌ 编译错误！String 不能按索引取
    for ch in cn.chars() {
        // 按"字符"遍历（对标 Java codePoint 遍历）
        print!("{ch} ");
    }
    println!();
    for byte in cn.bytes() {
        // 按字节遍历（对标 getBytes）
        print!("{byte:x} ");
    }
    println!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn string_is_growable_str_is_view() {
        let mut owned = String::from("hello");
        owned.push_str(" world");
        assert_eq!(owned, "hello world");

        let view: &str = &owned[6..]; // "world"
        assert_eq!(view, "world");
    }

    #[test]
    fn format_does_not_move() {
        let a = String::from("a");
        let b = String::from("b");
        let c = format!("{a}{b}"); // a、b 还能用
        assert_eq!(c, "ab");
        assert_eq!(a, "a");
    }

    #[test]
    fn parse_returns_result_not_exception() {
        let ok: i32 = "42".parse().unwrap();
        assert_eq!(ok, 42);
        assert!("abc".parse::<i32>().is_err()); // 不抛 NumberFormatException
    }

    #[test]
    fn utf8_length_vs_chars() {
        let s = "中文";
        assert_eq!(s.len(), 6); // 字节数：每个汉字 3 字节
        assert_eq!(s.chars().count(), 2); // 字符数
    }

    #[test]
    fn join_and_split() {
        let v = vec!["a", "b", "c"];
        assert_eq!(v.join(","), "a,b,c");
        let parts: Vec<&str> = "a,b,c".split(',').collect();
        assert_eq!(parts, vec!["a", "b", "c"]);
    }
}
