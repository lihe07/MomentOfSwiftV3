# Collection: Files

所有存储在OSS上的对象都得在这里有个映射

## 数据结构

必备字段

- `id: String` - 生成的uuid v4

- `name: String` - 原始文件名

- `path: String` - OSS路径

  > ​	备注：该路径必须随机分配，防止出现重复，推荐与id保持一致

- `last_modified_time: ISODate ` - 上次修改时间

  > ​	备注：对任何字段或文件本身的更改都会导致这个字段变化

- `created_time: ISODate` - 该映射创建的时间

  > ​	备注：一旦创建就不再修改

- `mime_type: String` - 文件的Mime Type

  > ​	备注：在每次修改文件 / 文件名时同步更新

- `owner: String` - 所有者的id，与 **users.id** 软关联

  > ​	备注：当某个user被删除时，应对其关联的files予以销毁（同步至OSS）

## 公共方法

暂时没有
