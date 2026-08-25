# 我的 AI 工作台

本地优先的个人 AI 产出与知识资产工作台。

## 目标

- 用卡片统一管理技能、知识、用户画像/偏好、AI 产出和项目资料
- Markdown 保存正文，SQLite 保存索引、标签、来源和搜索数据
- 本地离线可用，原始文件和工作台副本同时保留
- 为本机 AI 助手提供只读查询和新增卡片 API
- 个人数据与代码分离，个人数据默认不进入 Git

## 目录

- 代码：`C:\Users\86182\Desktop\工作台`
- 数据：`D:\工作台数据`
- 卡片正文：`D:\工作台数据\cards`
- 原始文件副本：`D:\工作台数据\originals`
- 视觉资源：`D:\工作台数据\assets`
- SQLite 索引：`D:\工作台数据\workbench.db`

## 开发

```bash
npm install
npm run build
npm run tauri:dev
```

## 本地 API

桌面应用启动后监听 `127.0.0.1:37821`：

- `GET /api/health`
- `GET /api/cards?query=关键词&type=knowledge&tag=Python`
- `GET /api/cards/:id`
- `POST /api/cards`（只新增，不支持修改/删除）

API 只绑定本机回环地址，不暴露到局域网。
