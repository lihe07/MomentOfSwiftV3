# DataModel - `data_model`

主cargo项目，包含数据模型的规定及相关方法的实现

**备注**：`/src/config.rs`由于涉及密钥等内容，不会在版本管理中体现

## 结构

- [`/src/`](./src/README.md): Source Code
- `/Cargo.toml`: Project Configuration

## 主要上游项目

- bson：serde的Binary JSON扩展
- serde：序列化和反序列化框架
- serde_json：serde的JSON扩展
- uuid：完善的UUID
- mongodm：轻量的MongoDB ODM
- r-cache：高性能线程安全的内存缓存