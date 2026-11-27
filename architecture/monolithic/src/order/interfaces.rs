//! order 接口层（对标 com.example.monolithic.order.interfaces.web）
//! 控制器 + 请求对象。只做参数组装与结果转换，业务在 application 层。
//! 真实 HTTP 版本见 web 模块（axum），这里用方法调用模拟。

use std::rc::Rc;

use super::application::{
    CancelOrderCommand, CreateOrderCommand, OrderDto, OrderLine, OrderService, PayOrderCommand,
};
use crate::shared::DomainResult;

/// 请求对象（对标 interfaces.web.CreateOrderRequest）
#[derive(Debug, Clone)]
pub struct CreateOrderRequest {
    pub customer: String,
    pub product_id: String,
    pub quantity: i32,
}

/// 接口层控制器（对标 @RestController 的 OrderController）
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
