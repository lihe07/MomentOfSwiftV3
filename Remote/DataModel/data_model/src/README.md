# DataModel - `data_model` - Source Code

项目源码

## 结构

- `/src/lib.rs`: 对外接口整理
- `/src/config.rs`: 数据库及资产配置，不在Github上体现
- `/src/collections`: 所有集合对应的数据模型
  - `mod.rs`: 对外接口整理，部分共享内容
    - [x] 初始化全局缓存对象`CACHE`
    - [x] 定义Trait `ById`
    - [x] 定义Trait `ToJson`
    - [ ] 复导出所有子模块中的数据模型结构体
  - `_template.rs`: 不包括在代码树中，仅用做参考
  - `files.rs`: 对应数据库中的`files`集合
    - [x] 模型结构体定义
    - [ ] 编写单元测试
  - **TODO 待完善**