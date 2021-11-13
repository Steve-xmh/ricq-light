# RS-QQ
qq-android协议的rust实现 移植于MiraiGo


# 已完成功能/开发计划
#### 登录
- [x] 账号密码登录
- [x] 二维码登录
- [x] 验证码提交
- [x] 设备锁验证
- [x] 错误信息解析

#### 消息类型
- [x] 文本
- [x] 表情
- [x] At
- [ ] 图片
- [ ] 语音
- [ ] 回复
- [ ] 长消息(仅群聊/私聊)
- [ ] 链接分享
- [ ] 小程序(暂只支持RAW)
- [ ] 短视频
- [ ] 合并转发
- [ ] 群文件(上传与接收信息)

#### 事件
- [x] 群消息
- [ ] 好友消息
- [ ] 临时会话消息
- [ ] 登录号加群
- [ ] 登录号退群(包含T出)
- [ ] 新成员进群/退群
- [ ] 群/好友消息撤回 
- [ ] 群禁言
- [ ] 群成员权限变更
- [ ] 收到邀请进群通知
- [ ] 收到其他用户进群请求
- [ ] 新好友
- [ ] 新好友请求
- [ ] 客户端离线
- [ ] 群提示 (戳一戳/运气王等) 

#### 主动操作
> 为防止滥用，将不支持主动邀请新成员进群

- [x] 发送群消息
- [x] 获取/刷新群列表
- [x] 获取/刷新群成员列表
- [x] 获取/刷新好友列表
- [ ] 发送好友消息
- [ ] 发送临时会话消息
- [ ] 获取群荣誉 (龙王/群聊火焰等)
- [ ] 处理加群请求
- [ ] 处理被邀请加群请求
- [ ] 处理好友请求
- [ ] 撤回群消息
- [ ] 群公告设置
- [ ] 获取群文件下载链接
- [ ] 群设置 (全体禁言/群名)
- [ ] 修改群成员Card
- [ ] 修改群成员头衔
- [ ] ~~群成员邀请~~
- [ ] 群成员禁言/解除禁言
- [ ] T出群成员
- [ ] 戳一戳群友
- [ ] 获取陌生人信息

#### 敏感操作
> 由于[QQ钱包支付用户服务协议](https://www.tenpay.com/v2/html5/basic/public/agreement/protocol_mqq_pay.shtml), 将不支持一切有关QQ钱包的协议

>4.13 您不得利用本服务实施下列任一的行为：
>\
>     （9） **侵害QQ钱包支付服务系統；**

- [ ] ~~QQ钱包协议(收款/付款等)~~