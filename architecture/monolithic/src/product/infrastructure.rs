//! product 基础设施层（对标 com.example.monolithic.product.infrastructure）

use std::cell::RefCell;
use std::collections::HashMap;

use super::domain::{Product, ProductRepository};

/// 内存仓储实现（对标 ProductJpaRepository + ProductRepositoryImpl）
/// 真实项目换成 sqlx/diesel 查库（见 db 模块）
/// RefCell 提供"内部可变性"：所有方法签名都是 &self，但内部存储可以修改
pub struct InMemoryProductRepository {
    products: RefCell<HashMap<String, Product>>,
}

impl InMemoryProductRepository {
    pub fn new() -> Self {
        Self {
            products: RefCell::new(HashMap::new()),
        }
    }
}

impl ProductRepository for InMemoryProductRepository {
    fn save(&self, product: &Product) {
        self.products
            .borrow_mut()
            .insert(product.id.clone(), product.clone());
    }

    fn find_by_id(&self, id: &str) -> Option<Product> {
        self.products.borrow().get(id).cloned()
    }

    fn find_all(&self) -> Vec<Product> {
        self.products.borrow().values().cloned().collect()
    }
}
