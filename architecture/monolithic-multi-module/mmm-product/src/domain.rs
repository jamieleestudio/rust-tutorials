//! mmm-product 领域层（对标 com.example.mmm.product.domain）

/// 商品实体（对标 domain.Product）
#[derive(Debug, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub stock: i32,
}

/// 仓储接口（对标 domain.ProductRepository）
pub trait ProductRepository {
    fn save(&self, product: &Product);
    fn find_by_id(&self, id: &str) -> Option<Product>;
    fn find_all(&self) -> Vec<Product>;
}
