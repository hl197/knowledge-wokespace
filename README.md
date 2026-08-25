# Knowledge Workspace

本地优先的个人 AI 产出与知识资产工作台框架。

## 目标

- 用卡片统一管理技能、知识、AI 产出和项目资料；
- Markdown 保存正文，SQLite 保存索引、标签、来源和搜索数据；
- 本地离线可用，原始文件和工作台副本分离保存；
- 为本机 AI 助手提供查询、新增和关系管理能力；
- 个人数据与代码分离，真实数据默认不进入 Git。

## 技术栈

```text
React + TypeScript + Vite + Tauri
SQLite + Markdown
Canvas 2D / CSS Motion
Lucide React
```

## 开发

```bash
npm install
npm run build
npm run tauri:dev
```

## 本地数据

运行时数据目录由本地配置决定，不写入本仓库。请根据本机环境配置数据路径和凭据；不要将真实数据库、Markdown 卡片、原始文件或 `.env` 文件提交到 Git。

## 本地 API

桌面应用启动后使用本机回环地址提供 API，例如：

```text
GET  /api/health
GET  /api/cards
GET  /api/cards/:id
POST /api/cards
```

API 只绑定本机回环地址，不暴露到局域网。

## 设计资源

本项目包含通用前端设计文档和脱敏实验草图。第三方资源只按需研究和改造，不整套复制品牌页面或私有资产。

## 数据安全

- 不提交真实个人数据；
- 不提交数据库、原始文件、凭据、Token 或私有配置；
- GitHub 只保存脱敏后的代码框架、文档、测试和示例资源；
- 生产或个人数据应保存在本机受保护目录。
