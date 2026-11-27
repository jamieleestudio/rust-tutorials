//! shared 模块：跨限界上下文的通用构件
//! 对标 java: com.example.monolithic.shared 包
//! （DomainEvent、AggregateRoot、DomainException 体系、IdGenerator）

use std::fmt;

/// 领域事件（对标 shared.DomainEvent 抽象类）
/// Java 用继承；Rust 用 trait。要求 Debug 是为了方便日志打印。
pub trait DomainEvent: fmt::Debug {
    /// 事件类型名（对标 getClass().getSimpleName()）
    fn event_type(&self) -> &'static str;
    /// 事件关联的聚合根 ID
    fn aggregate_id(&self) -> &str;
}

/// 聚合根标记（对标 shared.AggregateRoot）
pub trait AggregateRoot {
    fn id(&self) -> &str;
}

/// 领域错误体系：Rust 不用异常继承，用一个错误枚举表达所有业务失败
///
/// 对标 java 异常体系：
/// ```text
/// DomainException                       → enum DomainError
/// ├── EntityNotFoundException           → DomainError::NotFound
/// └── BusinessRuleViolationException    → DomainError::BusinessRuleViolation
/// ```
///
/// 关键差异：Java 靠 throw 冒泡；Rust 靠 `Result<T, E>` + `?` 运算符传播，
/// 且编译器强制每个调用方处理错误，不可能"忘记 catch"。
#[derive(Debug, Clone, PartialEq)]
pub enum DomainError {
    /// 对标 EntityNotFoundException
    NotFound(String),
    /// 对标 BusinessRuleViolationException
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

// 实现 std::error::Error，可与 ? 运算符及 anyhow 等生态库无缝协作
impl std::error::Error for DomainError {}

impl DomainError {
    /// 接口层把错误映射为 HTTP 状态码
    /// 对标 GlobalExceptionHandler 的 @ExceptionHandler + ResponseEntity.status
    pub fn to_http_status(&self) -> u16 {
        match self {
            DomainError::NotFound(_) => 404,
            DomainError::BusinessRuleViolation(_) => 422, // Unprocessable Entity
        }
    }
}

/// 统一结果类型别名（对标方法签名 throws DomainException）
pub type DomainResult<T> = Result<T, DomainError>;

/// ID 生成器（对标 shared.IdGenerator）
/// 这里用原子计数器演示；真实项目换 uuid crate
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

    #[test]
    fn error_maps_to_http_status() {
        assert_eq!(DomainError::NotFound("x".into()).to_http_status(), 404);
        assert_eq!(
            DomainError::BusinessRuleViolation("y".into()).to_http_status(),
            422
        );
    }
}
