# MomentOfSwiftV3
遇见雨燕（Moment of Swift）完整系统 v3.x

## 系统架构

- **Local**：前端 / 本地程序
  - [ ] **SSOWeb**：Web端 登录及注册
  - [ ] **DashboardWeb**：Web端 控制台
  - [ ] **LabelWeb**：Web端 检测及标注
  - [ ] **CommunityWeb**：Web端 社区
- **Remote**：后端 / 服务端程序
  - [ ] **SSOApi**：登录及注册
  - [ ] **StorageApi**：访问及操作云COS
  - [ ] **DashboardApi**：大部分数据交互
  - [ ] **LabelApi**：运行模型及标注数据
  - [ ] **LabelWS**：负责多人协作标注的通讯部分
  - [ ] **CommunityApi**：社区数据交互
  - [ ] **DataModel(lib)**：数据模型
  - [ ] **ApiBasics(lib)**：一些全部Api共用的内容
- **Docs**：设计文档

## 网络架构

### 托管及部署

全部后端服务均部署在[云函数 FC](https://www.aliyun.com/product/fc)上

全部前端服务均托管于开启了**静态网站模式的**[对象存储 OSS](https://www.aliyun.com/product/oss)上，每一个服务占用一个存储桶

全部服务共用的永久存储为[云数据库 MySQL](https://www.aliyun.com/product/rds/mysql)，统一使用一个数据库实例，*开发时使用本地数据库*

### 域及子域

- [x] ~~**www**：静态的门户页面（不在本项目内）~~
- [ ] **storage**：阿里云OSS
- [ ] **storage-api**：后端服务*StorageApi*
- [ ] **sso**：前端服务*SSOWeb*
- [ ] **sso-api**：后端服务*SSOApi*
- [ ] **dashboard**：前端服务*DashboardWeb*
- [ ] **dashboard-api**：后端服务*DashboardApi*
- [ ] **label**：前端服务*LabelWeb*
- [ ] **label-api**：后端服务*LabelApi*

## 时间线

- 2022/4/7：初步规划系统架构，创建存储库，创建本文档
- 2022/7/27：决定放弃MongoDB改用MySQL
