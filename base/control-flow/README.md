# control-flow

控制流：if 表达式、loop 带值、for 范围、match 完备匹配、if let / while let。

## Java 对照

| Rust | Java |
|---|---|
| `let v = if c { a } else { b };` | 三元运算符 `c ? a : b` |
| `loop { break v; }` | `while(true)` + 临时变量 |
| `for i in 1..=5` | `for (int i = 1; i <= 5; i++)` |
| `match`（必须穷尽） | `switch`（语句形式可不穷尽） |
| 枚举变体携带数据 + 解构 | enum + 构造参数 |
| `if let Some(v) = x` | `if (x.isPresent()) x.get()` |
| `while let Some(t) = stack.pop()` | `while (it.hasNext()) it.next()` |

## 运行

```powershell
cargo run -p control-flow
cargo test -p control-flow
```
