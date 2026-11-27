//! monolithic：单体架构 + DDD 四层（对标 java-tutorials/architecture/monolithic）
//!
//! Java 版是 Spring Boot + JPA + 十几个 package；
//! Rust 版是单 crate + 文件模块 + 内存仓储 —— 重点看**结构映射**，不比技术栈。
//!
#![allow(dead_code)] // 教学示例：保留与 java 版一致的完整 API 面（查询/事件字段等），部分未在演示流程中调用
//!
//! 分层对照（模块树 = 包结构，一层目录一个 java 包）：
//! ```text
//! src/main.rs               → MonolithicApplication + Spring IoC 装配
//! src/shared.rs             → com.example.monolithic.shared
//! src/product/
//!   ├ mod.rs                → 包根（re-export 出口）
//!   ├ domain.rs             → product.domain        实体 + 仓储接口
//!   ├ application.rs        → product.application   服务 + 命令/查询 + DTO
//!   └ infrastructure.rs     → product.infrastructure 仓储实现（对标 JPA）
//! src/order/                → 同上四层，另含 interfaces.rs → order.interfaces.web
//! src/payment/              → 同上三层
//! ```

mod order;
mod payment;
mod product;
mod shared;

use std::rc::Rc;

fn main() {
    println!("=== monolithic：单体架构（DDD 四层，单 crate 实现） ===\n");

    // ---------- 依赖装配：对标 Spring IoC 容器，这里是显式手动装配 ----------
    let ids = Rc::new(shared::IdGenerator::new());

    let product_repo: Rc<dyn product::ProductRepository> =
        Rc::new(product::InMemoryProductRepository::new());
    let products = Rc::new(product::ProductService::new(
        Rc::clone(&product_repo),
        Rc::clone(&ids),
    ));

    let order_repo: Rc<dyn order::OrderRepository> = Rc::new(order::InMemoryOrderRepository::new());
    let collector = Rc::new(order::EventCollector::new());
    // 注意：Rc<EventCollector> 可以在赋值处隐式"变大"成 Rc<dyn EventPublisher>（unsize 强制转换），
    // 但 Rc<T> 对 T 是不变的，所以不能写 Rc::clone(&collector) 再标注类型
    let publisher: Rc<dyn order::EventPublisher> = collector.clone();
    let orders = Rc::new(order::OrderService::new(
        Rc::clone(&order_repo),
        Rc::clone(&publisher),
        Rc::clone(&products),
        Rc::clone(&ids),
    ));

    let payment_repo: Rc<dyn payment::PaymentRepository> =
        Rc::new(payment::InMemoryPaymentRepository::new());
    let payments = payment::PaymentService::new(payment_repo, Rc::clone(&ids));

    let controller = order::OrderController::new(Rc::clone(&orders));

    // ---------- 1. 上架商品 ----------
    let keyboard = products.create("机械键盘", 399.0, 10).expect("商品创建失败");
    println!("1. 上架商品: {:?}\n", keyboard);

    // ---------- 2. 下单（interfaces → application → domain） ----------
    let req = order::CreateOrderRequest {
        customer: "张三".to_string(),
        product_id: keyboard.id.clone(),
        quantity: 2,
    };
    let order_dto = controller.create_order(&req).expect("下单失败");
    println!("2. 下单成功: {order_dto:?}\n");

    // ---------- 3. 发起并完成支付 ----------
    let pay = payments
        .create_payment(&order_dto.id, order_dto.total)
        .expect("创建支付单失败");
    let paid = payments.process_payment(&pay.id).expect("支付处理失败");
    println!("3. 支付单流转: {} -> {}\n", pay.status(), paid.status());

    // ---------- 4. 订单确认收款 ----------
    let paid_order = controller.pay_order(&order_dto.id).expect("订单支付失败");
    println!("4. 订单流转: {paid_order:?}\n");

    // ---------- 5. 业务规则拦截（对标 GlobalExceptionHandler 捕获异常转响应） ----------
    match controller.cancel_order(&order_dto.id) {
        Ok(_) => unreachable!("已支付订单取消应当被拒绝"),
        // match Result：对标 try/catch，但编译器强制你处理两种分支
        Err(e) => println!(
            "5. 业务规则拦截: {e} -> 接口层返回 HTTP {}（对标 GlobalExceptionHandler）\n",
            e.to_http_status()
        ),
    }

    // ---------- 6. 领域事件流（对标 Spring ApplicationEvent） ----------
    println!("6. 已发布领域事件: {:?}\n", collector.snapshot());

    // ---------- 7. 查询 ----------
    println!("7. 张三的订单: {:?}", orders.list_customer_orders("张三"));
    println!("   扣减后库存: {:?}", products.get(&keyboard.id));
}
