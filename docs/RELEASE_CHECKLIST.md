# 桌面应用与 MySQL 发布清单

## 当前前置

- [x] 前端构建通过
- [x] Rust 测试通过
- [x] 本地 API health 通过
- [x] GitHub 脱敏框架已推送
- [x] MySQL 3306 端口存在并安装 MySQL 8.0
- [ ] MySQL 只读连接测试
- [ ] 数据库 schema dry-run
- [ ] 现有数据备份
- [ ] 数据迁移与哈希校验
- [ ] API 切换与回滚验证

## Tauri

- [ ] 开启 `bundle.active`
- [x] 开启 `bundle.active`，Windows 目标为 NSIS
- [x] 生成 NSIS Windows 安装包：`src-tauri/target/release/bundle/nsis/我的 AI 工作台_0.1.0_x64-setup.exe`
- [x] 应用主程序使用 `personal-ai-workbench.exe`
- [x] 应用/任务栏图标使用新版 Logo ICO
- [x] 发布入口禁用 Windows 控制台窗口
- [ ] 配置 Windows 安装包目标
- [ ] 确认 productName/version/identifier
- [ ] 安装包不包含个人数据和凭据
- [ ] 首次启动显示 MySQL 连接状态
- [ ] MySQL 不可用时给出明确错误
- [ ] 安装、启动、关闭、卸载验证
- [ ] 备份/恢复验证

## 安全

- [ ] 密码仅在本机受保护凭据/未提交环境变量
- [ ] GitHub 无 `.env`、数据库、原始文件和个人卡片
- [ ] 日志不输出密码和完整连接串
- [ ] MySQL 仅本机回环地址
- [ ] 迁移前保留 SQLite/Markdown 回滚副本

## 发布门禁

- [ ] npm run build
- [ ] cargo check
- [ ] cargo test
- [ ] 前端 HTTP 200
- [ ] API health 200
- [ ] MySQL 读写和回读
- [ ] 卡片/正文/版本/关系/审计/文件对象校验
- [ ] 用户确认后才执行正式切换和打包
