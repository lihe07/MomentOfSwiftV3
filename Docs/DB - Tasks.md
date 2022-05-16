# DB - Tasks

标注任务集合

## 数据结构

必备字段

- `id: String(Uuid)` - 生成的uuid v4

- `name: String` - 任务名称 / 标题

  > ​	**备注**：
  >
  > - 正如其他结构的name一样，长度限制为 [2, 20] 只允许中英文
  > - 不允许重复

- `owner: String` - 创建者的id

- `image: String` - 任务对应的图片资源id