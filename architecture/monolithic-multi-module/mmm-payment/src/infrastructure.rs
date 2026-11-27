//! mmm-payment 基础设施层（对标 com.example.mmm.payment.infrastructure）

use std::cell::RefCell;
use std::collections::HashMap;

use super::domain::{Payment, PaymentRepository};

/// 内存仓储（对标 PaymentJpaRepository + PaymentRepositoryImpl）
pub struct InMemoryPaymentRepository {
    payments: RefCell<HashMap<String, Payment>>,
}

impl InMemoryPaymentRepository {
    pub fn new() -> Self {
        Self {
            payments: RefCell::new(HashMap::new()),
        }
    }
}

impl PaymentRepository for InMemoryPaymentRepository {
    fn save(&self, payment: &Payment) {
        self.payments
            .borrow_mut()
            .insert(payment.id.clone(), payment.clone());
    }

    fn find_by_id(&self, id: &str) -> Option<Payment> {
        self.payments.borrow().get(id).cloned()
    }

    fn find_all(&self) -> Vec<Payment> {
        self.payments.borrow().values().cloned().collect()
    }
}
