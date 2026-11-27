//! product 限界上下文（对标 com.example.monolithic.product 包）
//!
//! 模块树 = 包结构：
//! ```text
//! product/
//! ├ mod.rs            → 包根：声明子模块 + 对外 re-export（对标 import 的出口）
//! ├ domain.rs         → com.example.monolithic.product.domain
//! ├ application.rs    → com.example.monolithic.product.application
//! └ infrastructure.rs → com.example.monolithic.product.infrastructure
//! ```

mod application;
mod domain;
mod infrastructure;

// 包根做统一出口：外部用 crate::product::ProductService 这样的"扁平"路径
// （对标 java 的 import com.example.product.application.ProductService）
// 注：本 crate 是 bin，pub use 不构成对外 API，未用到的出口项会告警，故加 allow
#[allow(unused_imports)]
pub use application::{ProductDto, ProductService};
#[allow(unused_imports)]
pub use domain::{Product, ProductRepository};
#[allow(unused_imports)]
pub use infrastructure::InMemoryProductRepository;
