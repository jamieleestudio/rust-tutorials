# architecture —— 架构风格示例（对标 java-tutorials/architecture）

与 java-tutorials 相同的思路：**用同一个电商域（product / order / payment + shared-kernel）演示不同的架构风格**，
只换工程组织与部署形态，业务域不变，方便横向对比。

## 风格清单

| 目录 | 对标 java 模块 | 说明 | 状态 |
|---|---|---|---|
| `monolithic/` | `architecture/monolithic` | 单体 + DDD 四层，单 crate | ✅ |
| `monolithic-multi-module/` | `architecture/monolithic-multi-module` | 多模块单体：shared-kernel / product / order / payment / bootstrap 五个 crate | ✅ |
| `microservices/` | `architecture/microservices` | 微服务：每个服务独立进程，HTTP 边界（axum） | ⏳ |
| `distributed/` | `architecture/distributed` | 分布式拆分（远程调用 + 数据隔离） | ⏳ |
| `event-driven/` | `architecture/event-driven` | 事件驱动（事件总线解耦） | ⏳ |
| `cloud-native/` | `architecture/cloud-native` | 云原生（docker/helm/k8s 部署物） | ⏳ |
| `erp/` | `architecture/erp` | 平台层 / 模块层 / 应用层的大型工程组织 | ⏳ |
| `evolvable-architecture/` | `architecture/evolvable-architecture` | 演进式架构 | ⏳ |

## Rust 里"模块边界"和 Java 的本质区别

| Java | Rust |
|---|---|
| package 是软边界（反射、同 classloader 可绕过） | crate 是硬边界，编译器强制 |
| 依赖方向靠 ArchUnit / Maven 依赖约束 | 依赖方向由 `Cargo.toml` 声明，越界直接编译失败 |
| @Autowired 运行时装配 | 显式构造函数装配，无容器魔法 |
| Spring 事件 / MQ | trait 定义发布端口，实现可换（内存 / tokio broadcast / Kafka） |

## 运行

```powershell
cargo run -p monolithic     # 单体版
cargo run -p mmm-bootstrap  # 多模块单体版（同一套业务流程）
cargo test --workspace      # 两个风格的全部测试
```
