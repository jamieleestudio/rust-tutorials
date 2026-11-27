//! mmm-product 基础设施层（对标 com.example.mmm.product.infrastructure）

use std::cell::RefCell;
use std::collections::HashMap;

use super::domain::{Product, ProductRepository};

/// 内存仓储（对标 ProductJpaRepository + ProductRepositoryImpl）
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
