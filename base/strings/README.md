# strings

两种字符串：String（所有权）与 &str（借用切片）；UTF-8 与 UTF-16 的差异。

## Java 对照

| Rust | Java |
|---|---|
| `String`（可增长、拥有所有权） | `StringBuilder` |
| `&str`（只读切片、零拷贝视图） | （无对应，近似 substring 视图） |
| `format!("{a}{b}")` | `String.format` |
| `a + " " + &b`（左值被移动） | `a + " " + b`（无所有权语义） |
| `s.parse::<i32>()` 返回 `Result` | `Integer.parseInt` 抛异常 |
| `s.chars()`（Unicode 标量） | `codePointAt` 遍历 |
| `s.len()` 是**字节数** | `length()` 是 UTF-16 代码单元数 |
| 内部编码 UTF-8 | 内部编码 UTF-16 |

⚠️ 重点：Rust 字符串不能 `s[i]` 按索引取字符，因为 UTF-8 变长。

## 运行

```powershell
cargo run -p strings
cargo test -p strings
```
