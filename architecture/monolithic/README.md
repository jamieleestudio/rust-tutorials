# monolithic —— 单体架构 + DDD 四层

对标 java-tutorials `architecture/monolithic`（Spring Boot + JPA 版）。
同一套电商域：商品、订单、支付，全部在一个 crate 内完成。

## 分层映射（模块树 = 包结构）

| Rust（本 crate） | Java（com.example.monolithic） |
|---|---|
| `src/shared.rs` | `shared/` 包（DomainEvent、DomainException 体系、IdGenerator） |
| `src/product/mod.rs` | `product/` 包根（re-export 统一出口，对标 import） |
| `src/product/domain.rs` | `product/domain/`（Product、ProductRepository） |
| `src/product/application.rs` | `product/application/`（Service、DTO、命令/查询） |
| `src/product/infrastructure.rs` | `product/infrastructure/`（InMemory 仓储 对标 JPA 仓储） |
| `src/order/{mod,domain,application,infrastructure,interfaces}.rs` | `order/` 全部四层 + interfaces.web |
| `src/payment/{mod,domain,application,infrastructure}.rs` | `payment/` 全部四层 |
| `src/main.rs` 依赖装配 | Spring IoC 容器 |

拆成文件模块后可见性边界更严格：`Order.items/status` 私有字段只在 `order/domain.rs` 可见，
连同 crate 的 `application.rs` 都碰不到，只能走聚合的业务方法 —— DDD 的"聚合保护不变量"由编译器强制。

## 核心对照点

| Java 写法 | Rust 写法 |
|---|---|
| 异常继承体系 + throw | `enum DomainError` + `Result<T, E>` + `?` |
| `@RestController` | `OrderController`（HTTP 版在 web 模块用 axum） |
| Spring 容器装配 Bean | `main.rs` 显式构造 + `Rc` 共享 |
| JPA Repository | `trait Repository` + `InMemory*` 实现 |
| Spring ApplicationEvent | `trait EventPublisher` + `EventCollector` |
| enum OrderStatus + 方法判断 | `enum OrderStatus` + `match`（编译器强制穷尽） |
| ArchUnit 查违规依赖 | crate 内 module 边界 + 惯例注释 |

## 运行

```powershell
cargo run -p monolithic
cargo test -p monolithic
```

预期输出：上架商品 → 下单扣库存 → 支付 → 订单流转 → 取消被业务规则拦截（422）→ 领域事件列表。
