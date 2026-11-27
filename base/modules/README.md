# modules

模块系统：内联模块、文件模块、可见性（pub/pub(crate)/私有）、重导出。

这个 crate 演示 lib + bin 双 target 的组织方式：
- `src/lib.rs` —— 库入口（模块树根部）
- `src/customer.rs` —— 独立文件模块
- `src/main.rs` —— 可执行入口，通过 crate 名引用 lib

## Java 对照

| Rust | Java |
|---|---|
| crate | jar / 工程 |
| `mod foo` / `foo.rs` | `com.example.foo` 包 |
| `pub` / `pub(crate)` / 无修饰 | `public` / package-private / 包私有 |
| struct 字段默认私有 | 字段默认 package-private（惯例 private） |
| `pub use` 重导出 | 门面模式 / API 包装 |
| `crate::path` | 全限定名 |
| `super::path` | 上级包 |

> 💡 真实工程的文件模块划分示例见 `architecture/monolithic`：每个限界上下文一个目录（= 一个 java 包），按 domain/application/infrastructure/interfaces 拆文件。

## 运行

```powershell
cargo run -p modules
cargo test -p modules
```
