//! mmm-product：商品模块（对标 java mmm-product 模块）
//!
//! crate 内按 DDD 分层拆文件模块，对标 java 的包结构：
//! ```text
//! mmm-product/src/
//! ├ lib.rs              → 模块根（对标模块的"包根"）
//! ├ domain.rs           → com.example.mmm.product.domain
//! ├ application.rs      → com.example.mmm.product.application
//! └ infrastructure.rs   → com.example.mmm.product.infrastructure
//! ```
//! 结构与 monolithic 版一致，差异只有一点：shared 构件来自 mmm-shared-kernel crate

mod application;
mod domain;
mod infrastructure;

pub use application::{ProductDto, ProductService};
pub use domain::{Product, ProductRepository};
pub use infrastructure::InMemoryProductRepository;
