//! order 基础设施层（对标 com.example.monolithic.order.infrastructure）
//! 内存仓储 + 事件收集器

use std::cell::RefCell;
use std::collections::HashMap;

use super::application::EventPublisher;
use super::domain::{Order, OrderRepository};
use crate::shared::DomainEvent;

/// 内存订单仓储（对标 OrderJpaRepository + OrderRepositoryImpl）
pub struct InMemoryOrderRepository {
    orders: RefCell<HashMap<String, Order>>,
}

impl InMemoryOrderRepository {
    pub fn new() -> Self {
        Self {
            orders: RefCell::new(HashMap::new()),
        }
    }
}

impl OrderRepository for InMemoryOrderRepository {
    fn save(&self, order: &Order) {
        self.orders
            .borrow_mut()
            .insert(order.id.clone(), order.clone());
    }

    fn find_by_id(&self, id: &str) -> Option<Order> {
        self.orders.borrow().get(id).cloned()
    }

    fn find_by_customer(&self, customer: &str) -> Vec<Order> {
        self.orders
            .borrow()
            .values()
            .filter(|o| o.customer == customer)
            .cloned()
            .collect()
    }
}

/// 事件收集器（对标 SpringEventPublisherServiceImpl，基于 Spring 的 ApplicationEventPublisher）
/// 这里先收集到内存；微服务/事件驱动版本会替换为消息队列实现（见 mq 模块）
pub struct EventCollector {
    events: RefCell<Vec<Box<dyn DomainEvent>>>,
}

impl EventCollector {
    pub fn new() -> Self {
        Self {
            events: RefCell::new(Vec::new()),
        }
    }

    /// 查看已发布事件（演示与测试用）
    pub fn snapshot(&self) -> Vec<String> {
        self.events
            .borrow()
            .iter()
            .map(|e| format!("{}({})", e.event_type(), e.aggregate_id()))
            .collect()
    }
}

impl EventPublisher for EventCollector {
    fn publish(&self, event: Box<dyn DomainEvent>) {
        println!(
            "[事件发布] {} on {}",
            event.event_type(),
            event.aggregate_id()
        );
        self.events.borrow_mut().push(event);
    }
}
