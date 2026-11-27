# variables

变量与类型：不可变默认、mut、遮蔽、常量、标量类型、元组、数组。

## Java 对照

| Rust | Java |
|---|---|
| `let x = 5`（不可变） | `final int x = 5` |
| `let mut x = 5` | `int x = 5` |
| `let x = a; let x = b;`（遮蔽） | 无对应 |
| `const MAX: u32 = 1;` | `static final int MAX = 1;` |
| `var x = 42;`（推断） | `var x = 42;` |
| `char`（4 字节 Unicode 标量） | `char`（2 字节 UTF-16 代码单元） |
| `u8/u16/u32/u64` | ❌ 无无符号整数（除 `char`） |
| 元组 `(i32, f64)` | ❌ 无内建元组 |
| `[i32; 5]` 定长数组 | `int[5]` |

## 运行

```powershell
cargo run -p variables
cargo test -p variables
```
