# MySQL 存储迁移技术设计

> 状态：Proposed，等待用户确认后执行

## 1. 当前架构

```text
Tauri App
  ├─ React/Vite 前端
  ├─ 本地 API 127.0.0.1:37821
  └─ Rust 后端
       ├─ SQLite workbench.db
       ├─ Markdown cards/*.md
       ├─ originals/
       └─ backups/*.zip
```

## 2. 目标架构

```text
Tauri App
  ├─ React/Vite 前端
  ├─ 本地 API（端口待定）
  └─ Rust 后端
       └─ MySQL 8.0 127.0.0.1:3306
            ├─ 元数据
            ├─ 正文
            ├─ 版本
            ├─ 关系
            ├─ 审计
            ├─ 原始文件/附件 BLOB
            └─ 备份对象
```

MySQL 默认只绑定本机回环地址，不允许新增公网/局域网暴露。

## 3. 建议 schema

### cards

```text
id VARCHAR(128) PRIMARY KEY
 title VARCHAR(512) NOT NULL
 summary TEXT NOT NULL
 card_type VARCHAR(64) NOT NULL
 tags JSON NOT NULL
 source TEXT
 source_path TEXT
 visibility VARCHAR(64) NOT NULL
 status VARCHAR(32) NOT NULL
 favorite BOOLEAN NOT NULL DEFAULT FALSE
 created_at DATETIME(6) NOT NULL
 updated_at DATETIME(6) NOT NULL
 deleted_at DATETIME(6) NULL
```

### card_contents

```text
card_id VARCHAR(128) PRIMARY KEY
content LONGTEXT NOT NULL
content_sha256 CHAR(64) NOT NULL
updated_at DATETIME(6) NOT NULL
FOREIGN KEY card_id REFERENCES cards(id)
```

### card_versions

```text
id BIGINT AUTO_INCREMENT PRIMARY KEY
card_id VARCHAR(128) NOT NULL
title VARCHAR(512) NOT NULL
summary TEXT NOT NULL
tags JSON NOT NULL
status VARCHAR(32) NOT NULL
content LONGTEXT NOT NULL
content_sha256 CHAR(64) NOT NULL
created_at DATETIME(6) NOT NULL
FOREIGN KEY card_id REFERENCES cards(id)
```

### card_relations

```text
from_card_id VARCHAR(128) NOT NULL
to_card_id VARCHAR(128) NOT NULL
relation_type VARCHAR(64) NOT NULL
created_at DATETIME(6) NOT NULL
PRIMARY KEY(from_card_id,to_card_id,relation_type)
```

### audit_log

```text
id BIGINT AUTO_INCREMENT PRIMARY KEY
actor VARCHAR(128) NOT NULL
action VARCHAR(128) NOT NULL
target_id VARCHAR(128)
detail TEXT
created_at DATETIME(6) NOT NULL
```

### file_objects

用于原始文件、附件和备份对象：

```text
id VARCHAR(128) PRIMARY KEY
object_kind ENUM('original','attachment','backup') NOT NULL
file_name VARCHAR(512) NOT NULL
mime_type VARCHAR(255)
size_bytes BIGINT NOT NULL
sha256 CHAR(64) NOT NULL
content LONGBLOB NOT NULL
created_at DATETIME(6) NOT NULL
```

大于 MySQL 单行/客户端安全阈值的对象需要在迁移前确认 `max_allowed_packet`、分块策略和备份体积；不能直接假定全部 BLOB 迁移没有成本。

## 4. 迁移流程

```text
1. 停止写入或进入维护状态
2. 生成 SQLite + Markdown + originals + backups ZIP
3. 创建 MySQL database/schema（幂等）
4. 迁移 cards
5. 迁移正文并计算 SHA-256
6. 迁移 versions
7. 迁移 relations
8. 迁移 audit_log
9. 迁移原始文件/附件/备份对象
10. 按数量、哈希、抽样正文回读校验
11. dry-run API 读写
12. 用户确认后切换正式 API
13. 保留旧存储为只读回滚副本
```

## 5. 回滚

- MySQL 创建失败：不修改旧存储；
- 数据校验失败：不切换 API，保留失败日志和迁移副本；
- 切换后读取失败：恢复 API 到 SQLite/Markdown；
- 切换后写入失败：停止写入，记录冲突，不自动双写覆盖；
- 迁移成功且观察期结束后，旧存储仍保留，除非用户单独授权清理。

## 6. 连接配置

不把密码写入代码或仓库。推荐：

```text
WORKBENCH_MYSQL_HOST=127.0.0.1
WORKBENCH_MYSQL_PORT=3306
WORKBENCH_MYSQL_DATABASE=personal_ai_workbench
WORKBENCH_MYSQL_USER=root
WORKBENCH_MYSQL_PASSWORD=<本机受保护凭据，不提交>
```

桌面应用应从本机受保护配置读取密码，日志只显示 host/port/database，不显示完整连接串或密码。

## 7. 桌面应用风险

- Tauri 应用无法自动保证 MySQL 服务已启动；需要启动检查和可理解错误；
- 默认不主动新增 MySQL 常驻/开机自启；
- 数据库连接失败时不能静默创建新空库覆盖用户数据；
- 安装包不包含个人数据和凭据；
- 首次启动需显示数据连接状态和迁移/恢复入口。

## 8. 验收

- schema 可重复初始化；
- 迁移脚本支持 dry-run；
- cards/content/versions/relations/audit/file_objects 数量和哈希一致；
- API 读写、搜索、版本、关系、导入、备份和恢复通过；
- MySQL 停止时有明确错误，不破坏旧存储；
- Tauri bundle 安装、启动和卸载通过；
- 密码不出现在源码、Git、日志和错误提示中。
