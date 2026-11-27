//! mmm-payment：支付模块（对标 java mmm-payment 模块）
//!
//! crate 内按 DDD 分层拆文件模块：
//! ```text
//! mmm-payment/src/
//! ├ lib.rs              → 模块根
//! ├ domain.rs           → com.example.mmm.payment.domain
//! ├ application.rs      → com.example.mmm.payment.application
//! └ infrastructure.rs   → com.example.mmm.payment.infrastructure
//! ```

mod application;
mod domain;
mod infrastructure;

pub use application::PaymentService;
pub use domain::{Payment, PaymentRepository, PaymentStatus};
pub use infrastructure::InMemoryPaymentRepository;
