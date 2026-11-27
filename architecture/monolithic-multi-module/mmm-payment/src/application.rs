//! mmm-payment 应用层（对标 com.example.mmm.payment.application）

use std::rc::Rc;

use super::domain::{Payment, PaymentRepository};
use mmm_shared_kernel::{DomainError, DomainResult, IdGenerator};

/// 应用服务（对标 PaymentService + PaymentServiceImpl）
pub struct PaymentService {
    repo: Rc<dyn PaymentRepository>,
    ids: Rc<IdGenerator>,
}

impl PaymentService {
    pub fn new(repo: Rc<dyn PaymentRepository>, ids: Rc<IdGenerator>) -> Self {
        Self { repo, ids }
    }

    /// 对标 CreatePaymentCommand 处理
    pub fn create_payment(&self, order_id: &str, amount: f64) -> DomainResult<Payment> {
        let payment = Payment::create(self.ids.next_id(), order_id, amount)?;
        self.repo.save(&payment);
        Ok(payment)
    }

    /// 对标 ProcessPaymentCommand 处理
    pub fn process_payment(&self, payment_id: &str) -> DomainResult<Payment> {
        let mut payment = self
            .repo
            .find_by_id(payment_id)
            .ok_or_else(|| DomainError::NotFound(format!("支付单不存在: {payment_id}")))?;
        payment.mark_success()?;
        self.repo.save(&payment);
        Ok(payment)
    }

    /// 对标 GetPaymentByIdQuery 处理
    pub fn get_payment(&self, id: &str) -> DomainResult<Payment> {
        self.repo
            .find_by_id(id)
            .ok_or_else(|| DomainError::NotFound(format!("支付单不存在: {id}")))
    }

    /// 对标 GetAllPaymentsQuery 处理
    pub fn list_payments(&self) -> Vec<Payment> {
        self.repo.find_all()
    }
}
