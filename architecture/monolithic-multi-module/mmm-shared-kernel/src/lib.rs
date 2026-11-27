//! mmm-shared-kernel：共享内核
//! 对标 java mmm-shared-kernel 模块（com.example.mmm.shared 包）
//!
//! 放置所有模块都依赖的通用构件。
//! crate 边界 = 编译器强制的模块边界：别的 crate 只能用这里 pub 的内容。

use std::fmt;

/// 领域事件（对标 shared.DomainEvent）
pub trait DomainEvent: fmt::Debug {
    fn event_type(&self) -> &'static str;
    fn aggregate_id(&self) -> &str;
}

/// 聚合根标记（对标 shared.AggregateRoot）
pub trait AggregateRoot {
    fn id(&self) -> &str;
}

/// 领域错误体系（对标 shared.DomainException 体系）
/// Java 异常继承 → Rust 错误枚举 + Result 传播
#[derive(Debug, Clone, PartialEq)]
pub enum DomainError {
    NotFound(String),
    BusinessRuleViolation(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::NotFound(msg) => write!(f, "实体不存在: {msg}"),
            DomainError::BusinessRuleViolation(msg) => write!(f, "业务规则校验失败: {msg}"),
        }
    }
}

impl std::error::Error for DomainError {}

impl DomainError {
    /// 接口层错误 → HTTP 状态码（对标 GlobalExceptionHandler）
    pub fn to_http_status(&self) -> u16 {
        match self {
            DomainError::NotFound(_) => 404,
            DomainError::BusinessRuleViolation(_) => 422,
        }
    }
}

/// 统一结果别名
pub type DomainResult<T> = Result<T, DomainError>;

/// ID 生成器（对标 shared.IdGenerator）
pub struct IdGenerator {
    next: std::sync::atomic::AtomicU64,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self {
            next: std::sync::atomic::AtomicU64::new(1000),
        }
    }

    pub fn next_id(&self) -> String {
        use std::sync::atomic::Ordering;
        format!("ID-{}", self.next.fetch_add(1, Ordering::SeqCst))
    }
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_sequential() {
        let gen = IdGenerator::new();
        assert_eq!(gen.next_id(), "ID-1000");
        assert_eq!(gen.next_id(), "ID-1001");
    }
}
