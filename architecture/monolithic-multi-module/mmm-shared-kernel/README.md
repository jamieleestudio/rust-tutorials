# mmm-shared-kernel

对标 java-tutorials `architecture/monolithic-multi-module/mmm-shared-kernel`。

共享内核：DomainEvent、AggregateRoot、DomainError（异常体系）、IdGenerator。
所有其他 crate 的公共依赖，Cargo.toml 中零依赖 —— 依赖图的根。

| 内容 | Java 对标 |
|---|---|
| `trait DomainEvent` | `DomainEvent` 抽象类 |
| `trait AggregateRoot` | `AggregateRoot` 接口 |
| `enum DomainError` | `DomainException` 异常继承体系 |
| `struct IdGenerator` | `IdGenerator` 工具类 |
