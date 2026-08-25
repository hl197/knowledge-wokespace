# 桌面应用与 MySQL 存储迁移项目简报

> 状态：Discovery / 方案设计中
> 分级：L3（桌面应用 + 数据库迁移 + 个人数据 + 发布门禁）

## 一句话目标

将个人 AI 知识工作台交付为可安装的 Windows 桌面应用，并把知识库中的个人数据迁移到本机 MySQL，同时保留可验证、可回滚的旧存储副本。

## 当前事实

- 项目：React + TypeScript + Vite + Tauri 2
- 代码目录：当前工作台项目目录
- Tauri productName：我的 AI 工作台
- Tauri identifier：`com.hermes.personalaiworkbench`
- 当前 bundle：未开启
- 当前本地 API：`127.0.0.1:37821`
- 当前前端开发端口：`127.0.0.1:18427`
- 本机 MySQL：端口 `3306` 开放，MySQL Server 8.0 客户端和服务端可执行文件存在
- 当前旧存储：SQLite + Markdown + 本机原始文件/备份目录
- 当前工程基线：前端构建通过；Rust 测试 3 passed

## 目标用户与场景

用户本人在 Windows 桌面端浏览、搜索、阅读、整理和备份个人 AI 知识资产。

## 本次范围

- [ ] 设计并确认本机 MySQL schema
- [ ] 创建独立迁移脚本和回滚方案
- [ ] 备份现有 SQLite/Markdown/原始文件/备份
- [ ] 迁移卡片、正文、版本、关系、审计、原始文件、附件和备份
- [ ] API 切换到 MySQL，并保留旧存储回滚路径
- [ ] 开启 Tauri Windows bundle
- [ ] 完成安装包、首次启动、数据读写和恢复验收
- [ ] 更新 README、环境示例、迁移文档和发布清单

## 本次不做

- [ ] 不把个人数据上传到 GitHub
- [ ] 不把密码、Token 或连接 URL 写入仓库
- [ ] 不直接删除旧 SQLite/Markdown 数据
- [ ] 不在未验证迁移前切换正式 API
- [ ] 不把 MySQL 暴露到局域网或公网
- [ ] 不在聊天中接收或回显 MySQL 密码
- [ ] 不在没有回滚方案时打包发布

## 成功标准

- MySQL 本机连接成功，数据库和表结构可重复初始化；
- 现有数据迁移后卡片、正文、版本、关系、审计数量与旧存储一致；
- 原始文件、附件和备份可回读、可恢复；
- API 写入和读取通过真实运行态验证；
- 迁移失败可回滚到 SQLite/Markdown；
- Tauri Windows 安装包可安装、启动和访问本机数据；
- `npm run build`、`cargo check`、`cargo test` 通过；
- GitHub 只包含脱敏代码框架，不包含个人数据和凭据。

## 未决问题

- MySQL root 密码通过何种本机受保护方式提供：TBD
- 原始文件/附件/ZIP 是否全部作为 BLOB 存储，还是使用 MySQL 元数据 + 本机受保护对象存储：用户已倾向全部进入 MySQL，但需要确认容量和恢复方案
- 桌面应用使用内置本地 API 端口还是随机可用端口：TBD
- 是否需要 MySQL 开机自启：默认不主动新增常驻行为，TBD

## 第一阶段门禁

先完成 TECH_DESIGN 和迁移 dry-run；没有用户确认，不创建正式数据库、不切换 API、不打包发布。
