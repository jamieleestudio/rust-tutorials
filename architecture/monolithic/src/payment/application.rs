//! payment 应用层（对标 com.example.monolithic.payment.application）

use std::rc::Rc;

use super::domain::{Payment, PaymentRepository};
use crate::shared::{DomainError, DomainResult, IdGenerator};

/// 应用服务（对标 PaymentService + PaymentServiceImpl）
pub struct PaymentService {
    repo: Rc<dyn PaymentRepository>,
    ids: Rc<IdGenerator>,
}

impl PaymentService {
    pub fn new(repo: Rc<dyn PaymentRepository>, ids: Rc<IdGenerator>) -> Self {
        Self { repo, ids }
    }

    /// 对标 createPayment(CreatePaymentCommand)
    pub fn create_payment(&self, order_id: &str, amount: f64) -> DomainResult<Payment> {
        let payment = Payment::create(self.ids.next_id(), order_id, amount)?;
        self.repo.save(&payment);
        Ok(payment)
    }

    /// 对标 processPayment(ProcessPaymentCommand)
    pub fn process_payment(&self, payment_id: &str) -> DomainResult<Payment> {
        let mut payment = self
            .repo
            .find_by_id(payment_id)
            .ok_or_else(|| DomainError::NotFound(format!("支付单不存在: {payment_id}")))?;
        payment.mark_success()?;
        self.repo.save(&payment);
        Ok(payment)
    }

    /// 对标 getPaymentById(GetPaymentByIdQuery)
    pub fn get_payment(&self, id: &str) -> DomainResult<Payment> {
        self.repo
            .find_by_id(id)
            .ok_or_else(|| DomainError::NotFound(format!("支付单不存在: {id}")))
    }

    /// 对标 getAllPayments(GetAllPaymentsQuery)
    pub fn list_payments(&self) -> Vec<Payment> {
        self.repo.find_all()
    }
}
