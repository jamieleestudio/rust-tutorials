//! product 领域层（对标 com.example.monolithic.product.domain）

/// 商品实体（对标 domain.Product）
/// #[derive(Debug, Clone)] 属性宏生成的实现，对标 Lombok @Data/@Getter 等
#[derive(Debug, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub stock: i32,
}

/// 仓储接口：领域层声明、基础设施层实现（依赖倒置）
/// 对标 domain.ProductRepository 接口
pub trait ProductRepository {
    fn save(&self, product: &Product);
    fn find_by_id(&self, id: &str) -> Option<Product>;
    fn find_all(&self) -> Vec<Product>;
}
