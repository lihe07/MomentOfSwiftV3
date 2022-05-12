# Collection: Users

用户的集合

## 数据结构

必备字段

- `id: String(Uuid)` - 生成的uuid v4

- `name: String` - 用户的真实姓名

  > ​	**备注**：
  >
  > - 姓名的长度限制为 [2, 20] 只允许中英文
  > - 不允许重复

- `email: String` - 用户的邮箱

  > ​	**备注**：
  >
  > - 允许重复
  > - 根据正则表达校验，长度限制为 [1, 60]

- `password: String` - 用户的密码

- `permissions: Vec<String>` - 用户的权限

  > ​	**备注**：详细权限列表见[Permissions.md](./Permissions.md)

可选字段

- `avatar: String(Uuid)` - 头像对应的file，与Files集合关联

  > ​	**备注**：上传时检查mime_type是否以`image/`开头

- `profile: String` - 个人简介

  > ​	**备注**：长度限制 [1, 300]，支持markdown

- 待补充

## 公共方法

- by_email

  找到邮箱相同的用户，返回一个集合

- by_name

  如果有这个姓名的用户，则返回Some，否则为None

- name_exists

  姓名是否重复

