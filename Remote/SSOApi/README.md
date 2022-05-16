# SSOApi

单点登录及注册后端服务

## 结构

- [/src/](./src/README.md): Source Code
- /Cargo.toml: Project Configuration

## 主要上游项目

- [async-std](https://github.com/async-rs/async-std)
- [tide](https://github.com/http-rs/tide/)

## 接口文档

- [ ] **GET** `/users`：查询用户

  支持的Query：`name`，`id`

  **Session必须经过验证**

  *如果一个session多次反复请求查询，吊销它的verification*

- [ ] **POST** `/users`：提交注册表单

- [ ] **GET** `/session`：获取session的详细信息

- [ ] **PUT** `/session/verification`：申请验证session，如果有验证码，则提交

- [ ] **POST** `/verification`：申请验证码

- [ ] **GET** `/verification/<id>`：获取某个验证码的图片

- [ ] **HEAD** `/verification/<id>`：获取某个验证码的metadata

- [ ] **POST** `/login`：提交登录表单
