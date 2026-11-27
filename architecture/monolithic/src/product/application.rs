//! product 应用层（对标 com.example.monolithic.product.application）
//! 包含：DTO、应用服务、业务规则校验

use std::rc::Rc;

use super::domain::{Product, ProductRepository};
use crate::shared::{DomainError, DomainResult, IdGenerator};

/// 查询/展示对象（对标 application.dto.ProductDto）
#[derive(Debug, Clone)]
pub struct ProductDto {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub stock: i32,
}

impl From<Product> for ProductDto {
    // 对标 ProductDto.from(Product) 静态工厂方法
    fn from(p: Product) -> Self {
        Self {
            id: p.id,
            name: p.name,
            price: p.price,
            stock: p.stock,
        }
    }
}

/// 商品应用服务（对标 ProductService + ProductServiceImpl）
/// 字段是 trait object 的 Rc 智能指针：对标 Spring 注入的单例 Bean
/// Rc = 单线程引用计数共享；多线程场景换 Arc（见 concurrent 模块）
pub struct ProductService {
    repo: Rc<dyn ProductRepository>,
    ids: Rc<IdGenerator>,
}

impl ProductService {
    pub fn new(repo: Rc<dyn ProductRepository>, ids: Rc<IdGenerator>) -> Self {
        Self { repo, ids }
    }

    /// 对标 createProduct(CreateProductCommand)
    pub fn create(&self, name: &str, price: f64, stock: i32) -> DomainResult<Product> {
        // 业务规则校验：Java 抛异常，Rust 返回 Err
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

    /// 对标 getProductById(GetProductByIdQuery)
    pub fn get(&self, id: &str) -> DomainResult<ProductDto> {
        let product = self
            .repo
            .find_by_id(id)
            .ok_or_else(|| DomainError::NotFound(format!("商品不存在: {id}")))?;
        Ok(ProductDto::from(product))
    }

    /// 对标 getAllProducts(GetAllProductsQuery)
    pub fn list(&self) -> Vec<ProductDto> {
        self.repo.find_all().into_iter().map(ProductDto::from).collect()
    }

    /// 对标 deductStock(DeductStockCommand)
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

// ==================== 单元测试（对标 src/test/java 下的应用服务测试） ====================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::InMemoryProductRepository;

    fn service() -> ProductService {
        let repo: Rc<dyn ProductRepository> = Rc::new(InMemoryProductRepository::new());
        ProductService::new(repo, Rc::new(IdGenerator::new()))
    }

    #[test]
    fn product_name_required() {
        let s = service();
        assert!(matches!(
            s.create("  ", 10.0, 1),
            Err(DomainError::BusinessRuleViolation(_))
        ));
    }

    #[test]
    fn deduct_stock_checks_inventory() {
        let s = service();
        let p = s.create("键盘", 100.0, 2).unwrap();
        assert!(matches!(
            s.deduct_stock(&p.id, 3),
            Err(DomainError::BusinessRuleViolation(_))
        ));
        s.deduct_stock(&p.id, 2).unwrap();
        assert_eq!(s.get(&p.id).unwrap().stock, 0);
    }

    #[test]
    fn get_missing_product_is_not_found() {
        let s = service();
        assert!(matches!(s.get("NOPE"), Err(DomainError::NotFound(_))));
    }
}
