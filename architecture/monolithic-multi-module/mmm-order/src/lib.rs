//! mmm-order：订单模块（对标 java mmm-order 模块）
//!
//! 依赖：mmm-shared-kernel（通用构件）+ mmm-product（跨上下文调用商品服务）
//! 与 mmm-payment 之间零依赖 —— java 版靠 ArchUnit 检查这种模块边界，
//! Rust 版越界引用直接编译失败。
//! 注意：本 crate 的 Cargo.toml 没有 mmm-payment，想 use mmm_payment 都不可能。
//!
//! crate 内按 DDD 分层拆文件模块：
//! ```text
//! mmm-order/src/
//! ├ lib.rs              → 模块根
//! ├ domain.rs           → com.example.mmm.order.domain        聚合/状态机/事件/仓储接口
//! ├ application.rs      → com.example.mmm.order.application   命令/查询/DTO/服务/事件端口
//! ├ infrastructure.rs   → com.example.mmm.order.infrastructure 内存仓储 + 事件收集器
//! └ interfaces.rs       → com.example.mmm.order.interfaces.web 控制器
//! ```

mod application;
mod domain;
mod infrastructure;
mod interfaces;

pub use application::{
    CancelOrderCommand, CreateOrderCommand, EventPublisher, OrderDto, OrderLine, OrderLineDto,
    OrderService, PayOrderCommand,
};
pub use domain::{
    Order, OrderCancelledEvent, OrderCreatedEvent, OrderItem, OrderRepository, OrderStatus,
    OrderPaidEvent,
};
pub use infrastructure::{EventCollector, InMemoryOrderRepository};
pub use interfaces::{CreateOrderRequest, OrderController};
