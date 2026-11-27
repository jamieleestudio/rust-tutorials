# mmm-payment

对标 java-tutorials `architecture/monolithic-multi-module/mmm-payment`。

支付限界上下文：Payment 聚合（状态机 PENDING → SUCCESS）、PaymentRepository、PaymentService、InMemoryPaymentRepository。

依赖：仅 `mmm-shared-kernel`。与 mmm-order 之间没有任何编译期依赖 —— 订单模块不认识支付模块，编排由 mmm-bootstrap 完成。
