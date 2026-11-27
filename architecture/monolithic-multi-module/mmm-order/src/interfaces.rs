//! mmm-order 接口层（对标 com.example.mmm.order.interfaces.web）

use std::rc::Rc;

use super::application::{
    CancelOrderCommand, CreateOrderCommand, OrderDto, OrderLine, OrderService, PayOrderCommand,
};
use mmm_shared_kernel::DomainResult;

/// 请求对象（对标 interfaces.web.CreateOrderRequest）
#[derive(Debug, Clone)]
pub struct CreateOrderRequest {
    pub customer: String,
    pub product_id: String,
    pub quantity: i32,
}

/// 接口层控制器（对标 interfaces.web.OrderController）
pub struct OrderController {
    service: Rc<OrderService>,
}

impl OrderController {
    pub fn new(service: Rc<OrderService>) -> Self {
        Self { service }
    }

    pub fn create_order(&self, req: &CreateOrderRequest) -> DomainResult<OrderDto> {
        self.service.create_order(&CreateOrderCommand {
            customer: req.customer.clone(),
            lines: vec![OrderLine {
                product_id: req.product_id.clone(),
                quantity: req.quantity,
            }],
        })
    }

    pub fn pay_order(&self, order_id: &str) -> DomainResult<OrderDto> {
        self.service
            .pay_order(&PayOrderCommand { order_id: order_id.to_string() })
    }

    pub fn cancel_order(&self, order_id: &str) -> DomainResult<OrderDto> {
        self.service
            .cancel_order(&CancelOrderCommand { order_id: order_id.to_string() })
    }

    pub fn get_order(&self, order_id: &str) -> DomainResult<OrderDto> {
        self.service.get_order(order_id)
    }
}
