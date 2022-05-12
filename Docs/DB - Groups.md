# DB - Groups

调查小组集合

## 数据结构

必备字段

- `id: String(Uuid)` - 生成的uuid v4

- `name: String` - 小组名称

  > ​	**备注**：
  >
  > - 与user.name类似，长度限制为 [2, 20] 只允许中英文
  > - 不允许重复

- `category: i64` - 调查小组类型，0：未设置，1：古建筑调查组，2：现代建筑调查组，3：临时调查组

可选字段

- `cover: String(Uuid)` - 小组封面

- `profile: String` - 小组简介

  > ​	**备注**：长度限制类似user.profile [1, 300]，支持markdown

## 公共方法

- name_exists

  小组名称是否存在

