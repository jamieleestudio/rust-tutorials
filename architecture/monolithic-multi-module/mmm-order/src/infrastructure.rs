//! mmm-order 基础设施层（对标 com.example.mmm.order.infrastructure）

use std::cell::RefCell;
use std::collections::HashMap;

use super::application::EventPublisher;
use super::domain::{Order, OrderRepository};
use mmm_shared_kernel::DomainEvent;

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

/// 事件收集器（对标 SpringEventPublisherServiceImpl）
pub struct EventCollector {
    events: RefCell<Vec<Box<dyn DomainEvent>>>,
}

impl EventCollector {
    pub fn new() -> Self {
        Self {
            events: RefCell::new(Vec::new()),
        }
    }

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
