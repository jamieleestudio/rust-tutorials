# hello

第一个 Rust 程序：宏、格式化输出、函数定义、内建测试。

## Java 对照

| Rust | Java |
|---|---|
| `fn main()` | `public static void main(String[] args)` |
| `println!("{}")` | `System.out.printf` / `String.format` |
| `println!("{:?}")` | Lombok `@Data` 的 toString |
| `#[test]` + `assert_eq!` | JUnit 5 `@Test` + `assertEquals` |
| `/// 文档注释` | Javadoc |

## 运行

```powershell
cargo run -p hello
cargo test -p hello
```
