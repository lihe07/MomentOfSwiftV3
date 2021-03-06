# Permissions - 权限节点

`member.id`：小组编号为id的组员权限

`member.id.leader`：小组编号为id的组长权限

> 备注：含有`member.id`的权限只允许出现一个
>
> 获得`member.id.leader`时，将覆盖`member.id`

`admin`：管理员权限（可通过一切权限检查）

## 获得权限

通过应用邀请（也可以称为赋权），可以获得一定的权限

除`admin`外，无法强制赋权，所有邀请必须在被邀请者同意后才生效

- `member.id.leader`可以创建同id的`member.id.leader`和`member.id`

  *组长可以邀请其他用户成为自己组的组员*

  *组长可以提升本组用户为本组组长*

- `admin`可以创建包含任何权限节点的邀请，同时也可以管理任何用户（包含其他`admin`）的权限节点

  *管理员可以为用户直接赋权*

  *管理员可以创建任意邀请*

  *管理员可以取消其他管理员*

## 失去权限

- `member.id.leader`可以删除同组内的其他成员的`member.id`

  *组长可以将组员移出小组*

- `admin`可以删除任何用户的权限节点

  *管理员可以删除任何用户的权限*

  *管理员可以软封禁（失去填报的权限）任何用户*

