// 控制流：if / loop / while / for / match
// 对标 Java：if-else / do-while / while / for / switch

fn main() {
    // 1. if 是表达式，有返回值！对标 Java 三元运算符 a > b ? a : b
    let number = 7;
    let parity = if number % 2 == 0 { "偶数" } else { "奇数" };
    println!("{number} 是{parity}");

    // 2. loop：无限循环，且 break 可以带返回值（Java 需要额外变量）
    let mut count = 0;
    let doubled = loop {
        count += 1;
        if count == 10 {
            break count * 2; // break 带值，整个 loop 表达式的值就是它
        }
    };
    println!("loop 返回值 = {doubled}");

    // 3. while（对标 Java while，无区别）
    let mut n = 3;
    while n > 0 {
        println!("倒计时 {n}");
        n -= 1;
    }

    // 4. for + 范围（对标 Java for(int i = 1; i <= 5; i++)）
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();
    // 1..5 是左闭右开（对标 IntStream.range(1, 5)）
    // 反向与步进
    for i in (1..=5).rev() {
        print!("{i} ");
    }
    println!();
    // 遍历集合（对标 for-each）
    let names = vec!["Alice", "Bob"];
    for name in &names {
        println!("你好, {name}");
    }

    // 5. match：对标 switch，但能力天差地别
    //    - 必须穷尽所有分支（漏了编译不过，Java 的 switch 只有在表达式形式才要求）
    //    - 可以匹配枚举、结构体、元组、范围、解构，甚至守卫条件
    let message = Message::Move { x: 3, y: -1 };
    println!("{}", process(&message));
    println!("{}", process(&Message::Write(String::from("hello"))));
    println!("{}", process(&Message::ChangeColor(255, 0, 0)));
    println!("{}", process(&Message::Quit));

    // match 也是表达式（对标 Java 的 switch 表达式 yield）
    let code = 5;
    let level = match code {
        1..=3 => "低",
        4..=6 => "中",
        _ => "高", // _ 相当于 default
    };
    println!("code {code} 等级: {level}");

    // match 守卫（guard）：分支上加条件
    let age = 20;
    let ticket = match age {
        a if a < 6 => "免票",
        a if a < 18 => "半价",
        _ => "全价",
    };
    println!("age {age} => {ticket}");

    // 6. if let：只关心一种模式时的 match 简写
    let maybe: Option<i32> = Some(42);
    if let Some(v) = maybe {
        println!("拿到值 {v}");
    }
    // 对标 Java：if (maybe.isPresent()) { ... maybe.get() ... }

    // while let：循环解构（对标 Iterator.hasNext + next）
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("弹出 {top}");
    }
}

/// 演示用枚举：对标 Java 的 enum Message { ... }
/// Rust 枚举每个变体可以携带不同类型的数据（代数数据类型），
/// Java 的 enum 变体只能带构造参数，且要在一个类里写完
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

/// match 必须穷尽：漏掉任何变体编译报错，这是编译器保证的完备性
/// 对标 Java：switch 表达式要求 exhaustive，但语句形式不要求
fn process(msg: &Message) -> String {
    match msg {
        Message::Quit => "退出".to_string(),
        Message::Move { x, y } => format!("移动到 ({x}, {y})"), // 解构枚举字段
        Message::Write(s) => format!("写入: {s}"),
        Message::ChangeColor(r, g, b) => format!("改颜色 RGB({r}, {g}, {b})"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_is_expression() {
        let v = if 3 > 2 { 1 } else { 2 };
        assert_eq!(v, 1);
    }

    #[test]
    fn loop_break_with_value() {
        let mut c = 0;
        let r = loop {
            c += 1;
            if c == 5 {
                break c * 10;
            }
        };
        assert_eq!(r, 50);
    }

    #[test]
    fn match_is_exhaustive_and_returns_value() {
        assert_eq!(process(&Message::Quit), "退出");
        assert_eq!(
            process(&Message::Move { x: 1, y: 2 }),
            "移动到 (1, 2)"
        );
    }

    #[test]
    fn range_inclusive() {
        let mut sum = 0;
        for i in 1..=10 {
            sum += i;
        }
        assert_eq!(sum, 55);
    }
}
