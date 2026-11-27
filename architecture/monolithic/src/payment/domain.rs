//! payment 领域层（对标 com.example.monolithic.payment.domain）

use std::fmt;

use crate::shared::{DomainError, DomainResult};

/// 支付状态（对标 domain.PaymentStatus）
/// Failed 在本示例流程中未走到，但状态机定义保持完整（真实系统里退款/失败回调用得上）
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentStatus {
    Pending,
    Success,
    Failed,
}

impl fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PaymentStatus::Pending => "PENDING",
            PaymentStatus::Success => "SUCCESS",
            PaymentStatus::Failed => "FAILED",
        };
        f.write_str(s)
    }
}

/// 支付单聚合根（对标 domain.Payment）
/// status 私有字段：状态流转只能走 mark_success，保护状态机
#[derive(Debug, Clone)]
pub struct Payment {
    pub id: String,
    pub order_id: String,
    pub amount: f64,
    status: PaymentStatus,
}

impl Payment {
    /// 工厂方法（对标 Payment 创建逻辑）
    pub fn create(id: String, order_id: &str, amount: f64) -> DomainResult<Payment> {
        if amount <= 0.0 {
            return Err(DomainError::BusinessRuleViolation("支付金额必须大于 0".into()));
        }
        Ok(Payment {
            id,
            order_id: order_id.to_string(),
            amount,
            status: PaymentStatus::Pending,
        })
    }

    pub fn status(&self) -> PaymentStatus {
        self.status
    }

    /// 状态流转 PENDING -> SUCCESS（对标 Payment.markSuccess）
    pub fn mark_success(&mut self) -> DomainResult<()> {
        if self.status != PaymentStatus::Pending {
            return Err(DomainError::BusinessRuleViolation(format!(
                "仅 PENDING 状态支付单可处理，当前状态: {}",
                self.status
            )));
        }
        self.status = PaymentStatus::Success;
        Ok(())
    }
}

/// 仓储接口（对标 domain.PaymentRepository）
pub trait PaymentRepository {
    fn save(&self, payment: &Payment);
    fn find_by_id(&self, id: &str) -> Option<Payment>;
    fn find_all(&self) -> Vec<Payment>;
}

// ==================== 领域层单元测试 ====================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payment_amount_must_be_positive() {
        let p = Payment::create("ID-1".into(), "ID-2", 0.0);
        assert!(matches!(p, Err(DomainError::BusinessRuleViolation(_))));
    }

    #[test]
    fn only_pending_can_be_processed() {
        let mut p = Payment::create("ID-1".into(), "ID-2", 100.0).unwrap();
        p.mark_success().unwrap();
        assert!(matches!(
            p.mark_success(),
            Err(DomainError::BusinessRuleViolation(_))
        ));
    }
}
