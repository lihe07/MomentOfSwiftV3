# StorageApi

关联阿里云OSS和数据库的云函数

## 结构

- [`./src/`](./src/README.md): Source Code
- `/Cargo.toml`: Project Configuration

## 流程分析

访问 / 下载数据

```mermaid
sequenceDiagram
	Client -->> WebService:请求下载链接
	WebService ->> DB: 校验Session权限 + 检查文件信息
	DB ->> WebService: 用户信息 + 权限 + 文件信息
	note over WebService: 检查并计算签名
	WebService -->> Client: 返回下载链接 / 失败原因 
	
```

修改数据：重命名

```mermaid
sequenceDiagram
	Client -->> WebService: 请求重命名
    WebService ->> DB: 校验Session权限 + 检查文件信息
    DB ->> WebService: 用户信息 + 权限 + 文件信息
    WebService ->> OSS: 请求修改文件信息
    WebService -->> Client: 请求已发出(不保证完成)
    OSS ->> OSSTrigger: 某个文件发生修改
    OSSTrigger ->> DB: 修改数据库中文件信息
```

替换数据：上传新文件（PUT）

```mermaid
sequenceDiagram
	Client -->> WebService: 请求上传链接
	WebService ->> DB: 校验Session权限 + 检查文件信息
	DB ->> WebService: 用户信息 + 权限 + 文件信息
	note over WebService: 检查并计算
	WebService -->> Client: 返回上传链接(PUT) + 草稿文件ID / 失败原因
	Client -->> OSS: 上传文件
	OSS ->> OSSTrigger: 文件更改
	OSSTrigger ->> DB: 提升草稿文件 / 更新文件信息
```

## 配置及部署指南

> 该项目需与`OSSTrigger`配合部署

- 该项目针对阿里云Serverless开发，依赖于阿里云的**OSS API**
- 新建云函数时，选择`CustomRuntime -> Rust`
- 进入工作台，将已有代码全部删除
- 在Linux环境下运行`cargo build --release`，将生成的可执行文件重命名为`server`，并上传至云函数根目录
- 点击保存并部署

## 接口文档

- [ ] **GET** `/files/<id>`：获取一个文件的映射信息

  权限：必须登录，此外无特殊限制

