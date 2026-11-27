//! mmm-product 应用层（对标 com.example.mmm.product.application）

use std::rc::Rc;

use super::domain::{Product, ProductRepository};
use mmm_shared_kernel::{DomainError, DomainResult, IdGenerator};

/// DTO（对标 application.dto.ProductDto）
#[derive(Debug, Clone)]
pub struct ProductDto {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub stock: i32,
}

impl From<Product> for ProductDto {
    fn from(p: Product) -> Self {
        Self {
            id: p.id,
            name: p.name,
            price: p.price,
            stock: p.stock,
        }
    }
}

/// 应用服务（对标 ProductService + ProductServiceImpl）
pub struct ProductService {
    repo: Rc<dyn ProductRepository>,
    ids: Rc<IdGenerator>,
}

impl ProductService {
    pub fn new(repo: Rc<dyn ProductRepository>, ids: Rc<IdGenerator>) -> Self {
        Self { repo, ids }
    }

    /// 对标 CreateProductCommand 处理
    pub fn create(&self, name: &str, price: f64, stock: i32) -> DomainResult<Product> {
        if name.trim().is_empty() {
            return Err(DomainError::BusinessRuleViolation("商品名不能为空".into()));
        }
        if price < 0.0 {
            return Err(DomainError::BusinessRuleViolation("价格不能为负数".into()));
        }
        let product = Product {
            id: self.ids.next_id(),
            name: name.to_string(),
            price,
            stock,
        };
        self.repo.save(&product);
        Ok(product)
    }

    /// 对标 GetProductByIdQuery 处理
    pub fn get(&self, id: &str) -> DomainResult<ProductDto> {
        let product = self
            .repo
            .find_by_id(id)
            .ok_or_else(|| DomainError::NotFound(format!("商品不存在: {id}")))?;
        Ok(ProductDto::from(product))
    }

    /// 对标 GetAllProductsQuery 处理
    pub fn list(&self) -> Vec<ProductDto> {
        self.repo.find_all().into_iter().map(ProductDto::from).collect()
    }

    /// 对标 DeductStockCommand 处理
    pub fn deduct_stock(&self, id: &str, quantity: i32) -> DomainResult<()> {
        if quantity <= 0 {
            return Err(DomainError::BusinessRuleViolation(
                "扣减数量必须大于 0".into(),
            ));
        }
        let mut product = self
            .repo
            .find_by_id(id)
            .ok_or_else(|| DomainError::NotFound(format!("商品不存在: {id}")))?;
        if product.stock < quantity {
            return Err(DomainError::BusinessRuleViolation(format!(
                "库存不足: 商品[{}]剩余 {}，需要 {}",
                product.name, product.stock, quantity
            )));
        }
        product.stock -= quantity;
        self.repo.save(&product);
        Ok(())
    }
}
