# functions

函数：表达式即返回值、Option 替代 null、模拟默认参数/重载/可变参数、函数指针。

## Java 对照

| Rust | Java |
|---|---|
| `fn add(a: i32) -> i32 { a + b }` | `int add(int a, int b) { return a + b; }` |
| `if c { a } else { b }` | 三元运算符 |
| `Option<T>` 返回 | `Optional<T>` / 返回 null |
| `Option<&str>` 参数模拟默认值 | 方法重载 / 默认参数（无） |
| `&[&str]` 参数 | `String... args` |
| `fn(i32, i32) -> i32` 函数指针 | 方法引用 `Main::add` |
| ❌ 无方法重载 | `Overload(a)` / `Overload(a, b)` |

## 运行

```powershell
cargo run -p functions
cargo test -p functions
```
