// 变量与类型
// 对标 Java：变量声明、基本类型、final、static final

fn main() {
    // 1. Rust 变量默认不可变！这是 Rust 的核心设计
    //    对标 Java：final int x = 5;
    let x = 5;
    // x = 6; // ❌ 编译错误：cannot assign twice to immutable variable
    println!("x = {x}");

    // 2. mut 让变量可变（对标普通 Java 局部变量）
    let mut y = 5;
    println!("y 初值 = {y}");
    y = 6;
    println!("y 修改后 = {y}");

    // 3. 遮蔽（shadowing）：同名新变量覆盖旧变量，类型都可以不同
    //    Java 没有这个概念（Java 变量遮蔽只发生在作用域层面）
    let z = 5;
    let z = z + 1; // 还是 i32
    let z = z.to_string(); // 现在是 String，完全合法
    println!("z = {z}");

    // 4. 常量：编译期求值、必须显式标类型、命名用 SCREAMING_SNAKE_CASE
    //    对标 Java：static final int MAX_POINTS = 100_000;
    const MAX_POINTS: u32 = 100_000; // 数字下划线分隔，对标 Java 同样的语法
    println!("MAX_POINTS = {MAX_POINTS}");

    // 5. 标量类型（对标 Java 基本类型，但 Rust 有无符号整数！）
    let a: i32 = -2_147_483_648; // 对标 int（默认整数类型）
    let b: u64 = 18_446_744_073_709_551_615; // 对标 long，但 Java 没有无符号类型
    let c: f64 = 3.14; // 对标 double（Rust 浮点默认 f64）
    let d: bool = true; // 对标 boolean
    let e: char = '中'; // 对标 char，但 Rust char 是 4 字节 Unicode 标量值
                        // Java char 是 2 字节 UTF-16 代码单元，一个汉字可能要两个 char！
    println!("a={a} b={b} c={c} d={d} e={e}");

    // 6. 类型推断：不用写类型，编译器推断（对标 Java 10+ 的 var）
    let inferred = 42; // i32
    println!("inferred = {inferred}");

    // 7. 复合类型：元组（Java 没有内建元组）
    let tuple: (i32, f64, char) = (500, 6.4, 'z');
    let (t1, t2, t3) = tuple; // 解构赋值（Java 21 record 才有类似模式匹配）
    println!("t1={t1} t2={t2} t3={t3}，点访问: {}", tuple.0);

    // 8. 定长数组：类型和长度都是类型的一部分 [T; N]
    //    对标 Java：int[] arr = {1, 2, 3, 4, 5};
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros = [0; 3]; // [0, 0, 0]，对标 new int[3] 但带初值
    println!("arr[0]={} len={} zeros={:?}", arr[0], arr.len(), zeros);

    // 9. 整数溢出：debug 构建会 panic，release 构建回绕
    //    对标 Java：直接静默回绕（Math.addExact 才检查）
    let overflow_demo = 255u8;
    println!("u8 上限前: {overflow_demo}（+1 在 debug 模式会 panic）");
}

// ---- 单元测试 ----
#[cfg(test)]
mod tests {
    #[test]
    fn shadowing_allows_type_change() {
        let x = 5;
        let x = x.to_string();
        assert_eq!(x, "5");
    }

    #[test]
    fn char_is_four_bytes() {
        // Java 的 char 只有 2 字节，中文没问题但 emoji 要两个 char（代理对）
        // Rust 的 char 是 Unicode 标量值，emoji 一个 char 就够
        assert_eq!(std::mem::size_of::<char>(), 4);
        assert_eq!('中'.len_utf8(), 3);
    }

    #[test]
    fn tuple_destructure() {
        let t: (i32, &str) = (1, "one");
        let (n, s) = t;
        assert_eq!(n, 1);
        assert_eq!(s, "one");
    }

    #[test]
    fn array_has_fixed_length() {
        let arr = [1, 2, 3];
        assert_eq!(arr.len(), 3);
        assert_eq!(arr, [1, 2, 3]); // 数组支持整体比较
    }
}
