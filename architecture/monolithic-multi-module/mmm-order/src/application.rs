//! mmm-order 应用层（对标 com.example.mmm.order.application）
//! 命令/查询/DTO/事件端口/应用服务

use std::rc::Rc;

use super::domain::{
    Order, OrderCancelledEvent, OrderCreatedEvent, OrderItem, OrderPaidEvent, OrderRepository,
};
use mmm_product::ProductService;
use mmm_shared_kernel::{DomainError, DomainEvent, DomainResult, IdGenerator};

/// 命令对象（对标 application.command 包）
#[derive(Debug, Clone)]
pub struct OrderLine {
    pub product_id: String,
    pub quantity: i32,
}

#[derive(Debug, Clone)]
pub struct CreateOrderCommand {
    pub customer: String,
    pub lines: Vec<OrderLine>,
}

#[derive(Debug, Clone)]
pub struct PayOrderCommand {
    pub order_id: String,
}

#[derive(Debug, Clone)]
pub struct CancelOrderCommand {
    pub order_id: String,
}

/// DTO（对标 application.dto.OrderDto）
#[derive(Debug, Clone)]
pub struct OrderLineDto {
    pub product_id: String,
    pub product_name: String,
    pub quantity: i32,
    pub subtotal: f64,
}

#[derive(Debug, Clone)]
pub struct OrderDto {
    pub id: String,
    pub customer: String,
    pub status: String,
    pub total: f64,
    pub lines: Vec<OrderLineDto>,
}

impl From<&Order> for OrderDto {
    fn from(order: &Order) -> Self {
        Self {
            id: order.id.clone(),
            customer: order.customer.clone(),
            status: order.status().to_string(),
            total: order.total(),
            lines: order
                .items()
                .iter()
                .map(|i| OrderLineDto {
                    product_id: i.product_id.clone(),
                    product_name: i.product_name.clone(),
                    quantity: i.quantity,
                    subtotal: i.subtotal(),
                })
                .collect(),
        }
    }
}

/// 事件发布端口（对标 application.command.EventPublisherService）
pub trait EventPublisher {
    fn publish(&self, event: Box<dyn DomainEvent>);
}

/// 订单应用服务（对标 OrderService + OrderServiceImpl）
/// 注意 products 字段：来自 mmm-product crate 的服务，跨模块调用的注入点
pub struct OrderService {
    orders: Rc<dyn OrderRepository>,
    publisher: Rc<dyn EventPublisher>,
    products: Rc<ProductService>,
    ids: Rc<IdGenerator>,
}

impl OrderService {
    pub fn new(
        orders: Rc<dyn OrderRepository>,
        publisher: Rc<dyn EventPublisher>,
        products: Rc<ProductService>,
        ids: Rc<IdGenerator>,
    ) -> Self {
        Self {
            orders,
            publisher,
            products,
            ids,
        }
    }

    /// 对标 createOrder(CreateOrderCommand)
    pub fn create_order(&self, cmd: &CreateOrderCommand) -> DomainResult<OrderDto> {
        let mut items = Vec::new();
        for line in &cmd.lines {
            let product = self.products.get(&line.product_id)?;
            self.products.deduct_stock(&line.product_id, line.quantity)?;
            items.push(OrderItem {
                product_id: product.id.clone(),
                product_name: product.name.clone(),
                price: product.price,
                quantity: line.quantity,
            });
        }

        let order = Order::place(self.ids.next_id(), &cmd.customer, items)?;
        self.orders.save(&order);
        self.publisher.publish(Box::new(OrderCreatedEvent {
            order_id: order.id.clone(),
            customer: cmd.customer.clone(),
        }));
        Ok(OrderDto::from(&order))
    }

    /// 对标 payOrder(PayOrderCommand)
    pub fn pay_order(&self, cmd: &PayOrderCommand) -> DomainResult<OrderDto> {
        let mut order = self.load(&cmd.order_id)?;
        order.pay()?;
        self.orders.save(&order);
        self.publisher.publish(Box::new(OrderPaidEvent {
            order_id: order.id.clone(),
            amount: order.total(),
        }));
        Ok(OrderDto::from(&order))
    }

    /// 对标 cancelOrder(CancelOrderCommand)
    pub fn cancel_order(&self, cmd: &CancelOrderCommand) -> DomainResult<OrderDto> {
        let mut order = self.load(&cmd.order_id)?;
        order.cancel()?;
        self.orders.save(&order);
        self.publisher.publish(Box::new(OrderCancelledEvent {
            order_id: order.id.clone(),
        }));
        Ok(OrderDto::from(&order))
    }

    /// 对标 getOrderById(GetOrderByIdQuery)
    pub fn get_order(&self, id: &str) -> DomainResult<OrderDto> {
        let order = self.load(id)?;
        Ok(OrderDto::from(&order))
    }

    /// 对标 getOrdersByCustomer(GetOrdersByCustomerQuery)
    pub fn list_customer_orders(&self, customer: &str) -> Vec<OrderDto> {
        self.orders
            .find_by_customer(customer)
            .iter()
            .map(OrderDto::from)
            .collect()
    }

    fn load(&self, id: &str) -> DomainResult<Order> {
        self.orders
            .find_by_id(id)
            .ok_or_else(|| DomainError::NotFound(format!("订单不存在: {id}")))
    }
}

// ==================== 应用层单元测试（对标 java mmm-bootstrap 的 OrderTest） ====================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::{EventCollector, InMemoryOrderRepository};
    use mmm_product::{InMemoryProductRepository, ProductRepository};

    fn wire() -> (Rc<ProductService>, Rc<OrderService>, Rc<EventCollector>) {
        let ids = Rc::new(IdGenerator::new());
        let product_repo: Rc<dyn ProductRepository> = Rc::new(InMemoryProductRepository::new());
        let products = Rc::new(ProductService::new(Rc::clone(&product_repo), Rc::clone(&ids)));
        let order_repo: Rc<dyn OrderRepository> = Rc::new(InMemoryOrderRepository::new());
        let collector = Rc::new(EventCollector::new());
        let publisher: Rc<dyn EventPublisher> = collector.clone();
        let orders = Rc::new(OrderService::new(
            Rc::clone(&order_repo),
            Rc::clone(&publisher),
            Rc::clone(&products),
            Rc::clone(&ids),
        ));
        (products, orders, collector)
    }

    #[test]
    fn pay_then_cancel_is_rejected() {
        let (products, orders, _collector) = wire();
        let p = products.create("键盘", 100.0, 10).unwrap();
        let dto = orders
            .create_order(&CreateOrderCommand {
                customer: "张三".into(),
                lines: vec![OrderLine {
                    product_id: p.id.clone(),
                    quantity: 1,
                }],
            })
            .unwrap();
        let paid = orders
            .pay_order(&PayOrderCommand { order_id: dto.id.clone() })
            .unwrap();
        assert_eq!(paid.status, "PAID");
        assert!(matches!(
            orders.cancel_order(&CancelOrderCommand { order_id: dto.id }),
            Err(DomainError::BusinessRuleViolation(_))
        ));
    }

    #[test]
    fn insufficient_stock_is_rejected() {
        let (products, orders, _collector) = wire();
        let p = products.create("键盘", 100.0, 1).unwrap();
        let res = orders.create_order(&CreateOrderCommand {
            customer: "李四".into(),
            lines: vec![OrderLine {
                product_id: p.id,
                quantity: 5,
            }],
        });
        assert!(matches!(res, Err(DomainError::BusinessRuleViolation(_))));
    }
}
