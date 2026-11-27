# mmm-order

对标 java-tutorials `architecture/monolithic-multi-module/mmm-order`。

订单限界上下文：Order 聚合（Created → Paid/Cancelled 状态机）、领域事件（Created/Paid/Cancelled）、
OrderRepository、OrderService（CQRS 命令/查询）、EventCollector、OrderController。

## 依赖关系（Cargo 编译期强制）

```text
mmm-order ──> mmm-shared-kernel
mmm-order ──> mmm-product      （跨上下文调用：扣库存、查商品）
✗ mmm-order ──> mmm-payment    （无依赖！Cargo.toml 不声明就引用不到）
```

java 版需要 ArchUnit 的 `ArchitectureRulesTest` 来验证同样的事；Rust 版这条规则由 Cargo 天然保证 ——
想 `use mmm_payment::...` 会直接 `unresolved import` 编译错误。
