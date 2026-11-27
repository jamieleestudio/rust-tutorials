//! mmm-order 领域层（对标 com.example.mmm.order.domain）
//!
//! 可见性边界：Order 的 items/status 字段没有 pub，只在本文件可见 ——
//! 连同 crate 的 application.rs 也碰不到，只能通过聚合的业务方法变更状态。

use std::fmt;

use mmm_shared_kernel::{DomainError, DomainEvent, DomainResult};

/// 订单状态（对标 domain.OrderStatus）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Created,
    Paid,
    Cancelled,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OrderStatus::Created => "CREATED",
            OrderStatus::Paid => "PAID",
            OrderStatus::Cancelled => "CANCELLED",
        };
        f.write_str(s)
    }
}

/// 订单行（对标 domain.OrderItem）
#[derive(Debug, Clone)]
pub struct OrderItem {
    pub product_id: String,
    pub product_name: String,
    pub price: f64,
    pub quantity: i32,
}

impl OrderItem {
    pub fn subtotal(&self) -> f64 {
        self.price * self.quantity as f64
    }
}

/// 订单聚合根（对标 domain.Order）
#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub customer: String,
    items: Vec<OrderItem>, // 私有：仅 domain.rs 可见
    status: OrderStatus,   // 私有：仅 domain.rs 可见
}

impl Order {
    /// 工厂方法（对标 Order.place）
    pub fn place(id: String, customer: &str, items: Vec<OrderItem>) -> DomainResult<Order> {
        if customer.trim().is_empty() {
            return Err(DomainError::BusinessRuleViolation("下单客户不能为空".into()));
        }
        if items.is_empty() {
            return Err(DomainError::BusinessRuleViolation(
                "订单至少包含一个商品".into(),
            ));
        }
        if items.iter().any(|i| i.quantity <= 0) {
            return Err(DomainError::BusinessRuleViolation("商品数量必须大于 0".into()));
        }
        Ok(Order {
            id,
            customer: customer.to_string(),
            items,
            status: OrderStatus::Created,
        })
    }

    pub fn status(&self) -> OrderStatus {
        self.status
    }

    pub fn items(&self) -> &[OrderItem] {
        &self.items
    }

    pub fn total(&self) -> f64 {
        self.items.iter().map(OrderItem::subtotal).sum()
    }

    /// 状态机：仅 CREATED 可支付
    pub fn pay(&mut self) -> DomainResult<()> {
        if self.status != OrderStatus::Created {
            return Err(DomainError::BusinessRuleViolation(format!(
                "仅 CREATED 状态订单可支付，当前状态: {}",
                self.status
            )));
        }
        self.status = OrderStatus::Paid;
        Ok(())
    }

    /// 状态机：已支付不可取消
    pub fn cancel(&mut self) -> DomainResult<()> {
        match self.status {
            OrderStatus::Paid => Err(DomainError::BusinessRuleViolation(
                "已支付订单不可取消，请走退款流程".into(),
            )),
            OrderStatus::Cancelled => Err(DomainError::BusinessRuleViolation(
                "订单已经是取消状态".into(),
            )),
            OrderStatus::Created => {
                self.status = OrderStatus::Cancelled;
                Ok(())
            }
        }
    }
}

// ---------- 领域事件（对标 domain.event 包） ----------

#[derive(Debug, Clone)]
pub struct OrderCreatedEvent {
    pub order_id: String,
    pub customer: String,
}

#[derive(Debug, Clone)]
pub struct OrderPaidEvent {
    pub order_id: String,
    pub amount: f64,
}

#[derive(Debug, Clone)]
pub struct OrderCancelledEvent {
    pub order_id: String,
}

impl DomainEvent for OrderCreatedEvent {
    fn event_type(&self) -> &'static str {
        "OrderCreated"
    }
    fn aggregate_id(&self) -> &str {
        &self.order_id
    }
}

impl DomainEvent for OrderPaidEvent {
    fn event_type(&self) -> &'static str {
        "OrderPaid"
    }
    fn aggregate_id(&self) -> &str {
        &self.order_id
    }
}

impl DomainEvent for OrderCancelledEvent {
    fn event_type(&self) -> &'static str {
        "OrderCancelled"
    }
    fn aggregate_id(&self) -> &str {
        &self.order_id
    }
}

/// 仓储接口（对标 domain.OrderRepository）
pub trait OrderRepository {
    fn save(&self, order: &Order);
    fn find_by_id(&self, id: &str) -> Option<Order>;
    fn find_by_customer(&self, customer: &str) -> Vec<Order>;
}
