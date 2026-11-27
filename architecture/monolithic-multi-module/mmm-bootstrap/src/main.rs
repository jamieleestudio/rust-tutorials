//! mmm-bootstrap：启动与装配模块
//! 对标 java mmm-bootstrap（MonolithicMultiModuleApplication + GlobalExceptionHandler + 集成测试）
//!
#![allow(dead_code)] // 教学示例：保留与 java 版一致的完整 API 面
//!
//! 职责：
//! 1. 依赖装配 —— 对标 @SpringBootApplication 的组件扫描，这里是显式构造
//! 2. 编排跨模块业务流 —— order 支付后由 payment 模块完成收款
//! 3. 统一错误出口 —— 对标 GlobalExceptionHandler（@RestControllerAdvice）

use std::rc::Rc;

use mmm_order::{
    CreateOrderRequest, EventCollector, EventPublisher, InMemoryOrderRepository, OrderController,
    OrderRepository, OrderService,
};
use mmm_payment::{InMemoryPaymentRepository, PaymentRepository, PaymentService};
use mmm_product::{InMemoryProductRepository, ProductRepository, ProductService};
use mmm_shared_kernel::{DomainError, IdGenerator};

/// 统一错误出口（对标 GlobalExceptionHandler）
/// 接口层捕获领域错误，映射为"HTTP 状态码 + 消息"
/// 真实 HTTP 服务里对应一个 IntoResponse 的错误处理器（见 web 模块）
fn handle_error(stage: &str, e: &DomainError) {
    println!(
        "[GlobalExceptionHandler] {stage} 失败: {e} -> HTTP {}",
        e.to_http_status()
    );
}

fn main() {
    println!("=== monolithic-multi-module：多模块单体（5 crate，同一部署单元） ===\n");

    // ---------- 1. 装配共享内核 ----------
    let ids = Rc::new(IdGenerator::new());

    // ---------- 2. 装配商品模块 ----------
    let product_repo: Rc<dyn ProductRepository> = Rc::new(InMemoryProductRepository::new());
    let products = Rc::new(ProductService::new(Rc::clone(&product_repo), Rc::clone(&ids)));

    // ---------- 3. 装配订单模块（注入商品服务：跨模块协作点） ----------
    let order_repo: Rc<dyn OrderRepository> = Rc::new(InMemoryOrderRepository::new());
    let collector = Rc::new(EventCollector::new());
    let publisher: Rc<dyn EventPublisher> = collector.clone();
    let order_service = Rc::new(OrderService::new(
        Rc::clone(&order_repo),
        Rc::clone(&publisher),
        Rc::clone(&products),
        Rc::clone(&ids),
    ));
    let order_controller = OrderController::new(Rc::clone(&order_service));

    // ---------- 4. 装配支付模块 ----------
    let payment_repo: Rc<dyn PaymentRepository> = Rc::new(InMemoryPaymentRepository::new());
    let payments = Rc::new(PaymentService::new(payment_repo, Rc::clone(&ids)));

    // ---------- 5. 运行业务流 ----------
    // 5.1 上架
    let keyboard = products.create("机械键盘", 399.0, 10).expect("商品创建失败");
    println!("1. [mmm-product] 上架: {:?}\n", keyboard);

    // 5.2 下单（mmm-order 内部调用 mmm-product 扣库存）
    let req = CreateOrderRequest {
        customer: "张三".to_string(),
        product_id: keyboard.id.clone(),
        quantity: 2,
    };
    let order = match order_controller.create_order(&req) {
        Ok(dto) => {
            println!("2. [mmm-order] 下单成功: {dto:?}\n");
            dto
        }
        Err(e) => {
            handle_error("create_order", &e);
            return;
        }
    };

    // 5.3 收款（mmm-payment）→ 订单确认（mmm-order）
    //     跨模块编排发生在 bootstrap：因为 order 不依赖 payment，这正是解耦点
    let pay = match payments.create_payment(&order.id, order.total) {
        Ok(p) => p,
        Err(e) => {
            handle_error("create_payment", &e);
            return;
        }
    };
    match payments.process_payment(&pay.id) {
        Ok(done) => println!("3. [mmm-payment] 收款: {} -> {}\n", pay.status(), done.status()),
        Err(e) => {
            handle_error("process_payment", &e);
            return;
        }
    }
    let paid_order = match order_controller.pay_order(&order.id) {
        Ok(dto) => {
            println!("4. [mmm-order] 订单确认收款: {dto:?}\n");
            dto
        }
        Err(e) => {
            handle_error("pay_order", &e);
            return;
        }
    };

    // 5.4 业务规则拦截（对标 GlobalExceptionHandler 接住 BusinessRuleViolationException）
    if let Err(e) = order_controller.cancel_order(&order.id) {
        handle_error("cancel_order", &e);
    }
    println!();

    // 5.5 事件与查询
    println!("5. 领域事件: {:?}", collector.snapshot());
    println!("6. 模块间数据一致: 订单总额 = {:.2}, 支付金额 = {:.2}", paid_order.total, pay.amount);
}

// ---------- 集成测试（对标 mmm-bootstrap 的 MonolithicMultiModuleIntegrationTest） ----------
#[cfg(test)]
mod tests {
    use super::*;

    /// 跨 crate 装配（对标 Spring 容器把五个模块拼在一起）
    fn wire() -> (
        Rc<ProductService>,
        Rc<OrderController>,
        Rc<PaymentService>,
        Rc<EventCollector>,
    ) {
        let ids = Rc::new(IdGenerator::new());
        let product_repo: Rc<dyn ProductRepository> = Rc::new(InMemoryProductRepository::new());
        let products = Rc::new(ProductService::new(Rc::clone(&product_repo), Rc::clone(&ids)));

        let order_repo: Rc<dyn OrderRepository> = Rc::new(InMemoryOrderRepository::new());
        let collector = Rc::new(EventCollector::new());
        let publisher: Rc<dyn EventPublisher> = collector.clone();
        let service = Rc::new(OrderService::new(
            Rc::clone(&order_repo),
            Rc::clone(&publisher),
            Rc::clone(&products),
            Rc::clone(&ids),
        ));
        let controller = Rc::new(OrderController::new(Rc::clone(&service)));

        let payment_repo: Rc<dyn PaymentRepository> = Rc::new(InMemoryPaymentRepository::new());
        let payments = Rc::new(PaymentService::new(payment_repo, Rc::clone(&ids)));

        (products, controller, payments, collector)
    }

    #[test]
    fn full_flow_across_modules() {
        let (products, order_controller, payments, collector) = wire();

        // mmm-product：上架
        let p = products.create("显示器", 1299.0, 5).unwrap();

        // mmm-order：下单（内部调 mmm-product 扣库存）
        let order = order_controller
            .create_order(&CreateOrderRequest {
                customer: "张三".to_string(),
                product_id: p.id.clone(),
                quantity: 2,
            })
            .unwrap();
        assert_eq!(order.status, "CREATED");
        assert_eq!(products.get(&p.id).unwrap().stock, 3); // 库存被扣减

        // mmm-payment：收款
        let pay = payments.create_payment(&order.id, order.total).unwrap();
        let done = payments.process_payment(&pay.id).unwrap();
        assert_eq!(done.status().to_string(), "SUCCESS");

        // mmm-order：确认收款
        let paid = order_controller.pay_order(&order.id).unwrap();
        assert_eq!(paid.status, "PAID");

        // 业务规则：已支付不可取消
        assert!(matches!(
            order_controller.cancel_order(&order.id),
            Err(DomainError::BusinessRuleViolation(_))
        ));

        // 事件：Created + Paid 各一条
        assert_eq!(collector.snapshot().len(), 2);
    }
}
