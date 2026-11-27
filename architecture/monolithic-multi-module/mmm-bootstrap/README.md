# mmm-bootstrap

对标 java-tutorials `architecture/monolithic-multi-module/mmm-bootstrap`。

启动与装配模块（二进制 crate，依赖全部业务模块）：

| 职责 | Java 对标 |
|---|---|
| `main.rs` 里的装配代码 | `@SpringBootApplication` 组件扫描 |
| 跨模块编排（payment 收款 → order 确认） | Controller 编排多个 Service |
| `handle_error()` | `GlobalExceptionHandler`（@RestControllerAdvice） |
| `#[cfg(test)] mod tests` 集成测试 | `MonolithicMultiModuleIntegrationTest` |

运行：

```powershell
cargo run -p mmm-bootstrap
```

依赖图（编译期由 Cargo 强制，对标 Maven 依赖 + ArchUnit 检查）：

```text
                mmm-bootstrap
               /   |         \    \
              v    v          v    v
      mmm-product  mmm-order  mmm-payment
              \      |      /
               v     v     v
             mmm-shared-kernel
```
