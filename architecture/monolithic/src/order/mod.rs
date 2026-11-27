//! order 限界上下文（对标 com.example.monolithic.order 包）
//!
//! java 版四个 package 十几个类，这里拆成四个文件模块：
//! ```text
//! order/
//! ├ mod.rs             → 包根（re-export 统一出口）
//! ├ domain.rs          → order.domain        聚合/状态机/事件/仓储接口
//! ├ application.rs     → order.application   命令/查询/DTO/服务/事件端口
//! ├ infrastructure.rs  → order.infrastructure 内存仓储 + 事件收集器
//! └ interfaces.rs      → order.interfaces.web 控制器
//! ```

mod application;
mod domain;
mod infrastructure;
mod interfaces;

// 包根做统一出口（对标 java 的包级 import）；bin crate 中未用到的出口项会告警，故加 allow
#[allow(unused_imports)]
pub use application::{
    CancelOrderCommand, CreateOrderCommand, EventPublisher, OrderDto, OrderLine, OrderLineDto,
    OrderService, PayOrderCommand,
};
#[allow(unused_imports)]
pub use domain::{
    Order, OrderCancelledEvent, OrderCreatedEvent, OrderItem, OrderPaidEvent, OrderRepository,
    OrderStatus,
};
#[allow(unused_imports)]
pub use infrastructure::{EventCollector, InMemoryOrderRepository};
#[allow(unused_imports)]
pub use interfaces::{CreateOrderRequest, OrderController};
