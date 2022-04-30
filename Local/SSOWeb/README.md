# SSOWeb

系统单点登录的桌面端界面

## 功能和特性

- [x] 初始访问时更新session，如已经登陆则直接进入success阶段
- [x] 查询动作前申请authentication，如回绝则进入verify页面
- [x] 通过邮箱或者姓名找到指定用户，之后进入password阶段
- [ ] 如找到了多个用户，进入select阶段
- [ ] 获取后端的图片验证码，并在用户完成验证码后申请authentication，如成功则返回上一个阶段
- [ ] success页面指引用户到系统的其他部分
- [ ] register页面通过邀请码找到指定邀请（这一步需要申请authentication）
- [ ] 展示邀请内容，这一阶段允许用户返回重新输入邀请码
- [ ] 引导用户填写必填的个人信息（姓名，邮箱）
- [ ] 校验这些信息，如果通过则申请注册新账号，并跳转success页面

## 开发、编译、部署

安装依赖：`npm i`

打开开发服务器：`npm run dev`

编译：`npm run build`

