//! payment 限界上下文（对标 com.example.monolithic.payment 包）
//! 一个较简洁的上下文：domain / application / infrastructure 三层

mod application;
mod domain;
mod infrastructure;

// 包根做统一出口（对标 java 的包级 import）；bin crate 中未用到的出口项会告警，故加 allow
#[allow(unused_imports)]
pub use application::PaymentService;
#[allow(unused_imports)]
pub use domain::{Payment, PaymentRepository, PaymentStatus};
#[allow(unused_imports)]
pub use infrastructure::InMemoryPaymentRepository;
