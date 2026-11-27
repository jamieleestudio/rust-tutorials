# mmm-product

对标 java-tutorials `architecture/monolithic-multi-module/mmm-product`。

商品限界上下文：Product 实体、ProductRepository（trait）、ProductService（应用服务）、InMemoryProductRepository（基础设施）。

依赖：仅 `mmm-shared-kernel`。跨模块调用通过别的 crate 注入本 crate 的服务实例完成（见 mmm-bootstrap 装配）。
