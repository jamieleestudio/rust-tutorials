# rust-tutorials

Rust 学习指南，架构与 [java-tutorials](https://gitee.com/) 对齐：同样的模块划分思路，用 **Cargo Workspace** 对标 Maven 多模块聚合。

## 环境准备

```powershell
# 1. 安装 rustup（Rust 版本管理器，对标 SDKMAN/jabba）
winget install Rustlang.Rustup

# 2. 安装稳定版工具链（默认 MSVC 目标，需要 VS Build Tools 的链接器，rustup 会提示）
rustup default stable

# 3. 验证
rustc --version
cargo --version
```

## 架构对照：Maven vs Cargo

| java-tutorials | rust-tutorials | 说明 |
|---|---|---|
| 根 `pom.xml`（packaging=pom） | 根 `Cargo.toml`（`[workspace]` members） | 聚合 + 统一版本/依赖管理 |
| `<module>concurrent</module>` | `members = ["concurrent/*"]` | 按主题聚合子模块 |
| 子模块（如 `concurrent/volatile`） | 子 crate（如 `concurrent/mutex`） | 每个知识点独立编译、独立运行 |
| `src/main/java` | `src/`（`main.rs` / `lib.rs`） | 源码位置 |
| `src/test/java` + JUnit | `#[cfg(test)] mod tests` + `cargo test` | 测试是语言内建能力，无需依赖 |
| `dependencyManagement` | `[workspace.dependencies]` | 依赖版本统一收口 |
| `mvn test` | `cargo test --workspace` | 一键全量测试 |
| `mvn -pl <module>` | `cargo run -p <crate>` | 定位单个模块 |

## 模块总览

| 模块 | 对标 java-tutorials | 内容 | 状态 |
|---|---|---|---|
| `base/` | `base` | 语言基础：hello、variables、control-flow、functions、collections、strings、modules | ✅ |
| `architecture/` | `architecture` | 用同一电商域（product/order/payment + shared-kernel）演示各架构风格 | 🚧 |
| ├ `monolithic` | `architecture/monolithic` | 单体 + DDD 四层，单 crate 实现 | ✅ |
| ├ `monolithic-multi-module` | `architecture/monolithic-multi-module` | 多模块单体：`mmm-shared-kernel`/`mmm-product`/`mmm-order`/`mmm-payment`/`mmm-bootstrap` 五个 crate | ✅ |
| ├ `microservices` | `architecture/microservices` | 微服务：服务独立进程 + HTTP 边界（axum） | ⏳ |
| ├ `distributed` | `architecture/distributed` | 分布式拆分 | ⏳ |
| ├ `event-driven` | `architecture/event-driven` | 事件驱动 | ⏳ |
| ├ `cloud-native` | `architecture/cloud-native` | 云原生 + 部署（docker/helm/k8s） | ⏳ |
| ├ `erp` | `architecture/erp` | 平台/模块/应用三层的大型工程组织 | ⏳ |
| └ `evolvable-architecture` | `architecture/evolvable-architecture` | 演进式架构 | ⏳ |
| `ownership/` | —（Java 靠 GC，无对应） | 所有权、借用、切片、生命周期 | ⏳ Rust 专属 |
| `traits/` | 散落在 `base` | struct/enum、模式匹配、trait、泛型、错误处理 | ⏳ |
| `advanced/` | — | 智能指针、闭包与迭代器、宏、unsafe、测试组织 | ⏳ Rust 专属 |
| `concurrent/` | `concurrent` | threads、channels、mutex+arc、async/await（tokio） | ⏳ |
| `serialize/` | `serialize` | serde：JSON/YAML/CBOR | ⏳ |
| `web/` | `spring`/`web` | axum/actix-web（对标 Spring MVC） | ⏳ |
| `db/` | `orm`/`db` | sqlx/diesel（对标 JPA/MyBatis） | ⏳ |
| `rpc/` | `rpc` | tonic gRPC | ⏳ |
| `log/` | `log` | tracing/log（对标 slf4j/logback） | ⏳ |
| `cache/`、`mq/`、`ai/`、`expression/`、`format/`、`rule-engine/`、`distributed-infra/` | 同名模块 | 生态实践 | ⏳ |
| — | `spring`/`servlet`/`jakarta`/`jndi`/`bpm`/`office` | Java 专属体系，无直接对应 | ❌ 不移植 |

## 给 Java 开发者的心智映射

| Java | Rust |
|---|---|
| `interface` | `trait` |
| Maven 模块 | crate |
| `Optional<T>` | `Option<T>` |
| try/catch 异常 | `Result<T, E>` + `?` 运算符 |
| GC | 所有权/借用检查（编译期） |
| `synchronized` / `ConcurrentHashMap` | `Mutex` / `Arc`（无锁数据结构内置） |
| Spring IoC 容器 | 无容器：`Rc`/`Arc` + trait object 手动组装 |
| 注解（@Override 等） | 属性宏（#[derive(...)]、#[test]） |
| 泛型擦除 | 单态化（零开销） |

## 常用命令

```powershell
cargo build                # 编译全部
cargo test --workspace     # 全量测试
cargo run -p hello         # 运行某个示例
cargo run -p monolithic    # 运行单体架构示例
cargo run -p mmm-bootstrap # 运行多模块单体示例
cargo clippy               # 静态检查（对标 SpotBugs/Checkstyle）
cargo fmt                  # 格式化（对标 google-java-format）
```

> 💡 中文乱码：Windows 传统控制台默认 GBK 编码，先执行 `chcp 65001`（或用 Windows Terminal / IDE 内置控制台）再 `cargo run`。

## 学习路线

1. **base**：把 `base/` 下 7 个 crate 逐个 `cargo run -p <名字>` 跑一遍，对照 README 里的 Java 对照点
2. **architecture/monolithic**：看 DDD 四层如何在一个 crate 里用 module 表达，重点体会 trait（仓储接口）与 enum（错误体系）替代 Java 接口与异常继承
3. **architecture/monolithic-multi-module**：看 crate 边界如何物理隔离模块（编译器强制依赖方向，对标 Maven 依赖管理）
4. 之后按表推进：ownership → traits → advanced → concurrent → 生态模块
