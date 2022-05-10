# 数据库

项目统一使用**唯一**的**MongoDB**数据库，数据库主机需确保安全性，并保证数据库定期备份

## 命名规范

- 所有的集合必须以**可数名词复数**为名，全部字母均小写

  - :heavy_check_mark: files

  - :x: Files

  - :x: file

- 中间集合为下划线分割的两个集合名词

  - :heavy_check_mark: groups_users
  - :x: groups_mtm_users
  - :x: groups_to_users

- 待补充....

## 数据结构设计

- 如果某个数据需要id标识，**不得使用**MongoDB默认的`_id: ObjectId`参数

  > ​	数据库的`_id`是按顺序分配的，容易造成撞库
  >
  > ​	文档应有一个`id: String`字段，并使用`uuid::Uuid::new_v4()`生成ID

- 避免`field: [id1, id2, id3, ...]`的出现

  - 一对多：多包含单
  - 多对一：多包含单
  - 多对多：新表建关联

- 字段名称使用下划线分割法，整体为名词

- 字段命名不得含有集合名称

- 时间字段**必须**使用`ISODate`格式（UTC时间），不得使用其他格式存储时间戳

- 待补充....

## 数据结构方法

- 所有具备`id: String`字段的数据结构均需要实现`ById`

  这个trait包含三个方法：`by_id`，`sync`，`by_ids`，分别对应了查找和更新缓存