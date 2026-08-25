# 我的 AI 工作台 · 设计系统 DESIGN.md

> 版本：1.0
> 状态：正式
> 适用范围：前端视觉与交互实现
> 依据：FRONTEND-PRD.md、FRONTEND-COMPONENT-ATLAS.md、UI UX Pro Max 本地研究源（vendor/）
> 原则：不照搬品牌界面；真实个人数据不进入外部视觉服务；视觉层不侵入数据层。

---

## 1. 设计语言

东方数字知识博物馆：明亮、温暖、有艺术气息，并带科技细节。

- 每张卡片像一件可收藏、可探索的知识展品；
- 页面像一个有展厅、展柜、灯光和空间层次的数字博物馆；
- 东方元素采用原创提炼，不复制具体古建筑、名画、古籍或品牌界面；
- 不使用全屏沉重黑暗遮蔽内容；
- 避免普通 SaaS 卡片、无意义渐变、泛滥毛玻璃和模板化图标布局。

## 2. 设计 Token

### 2.1 字体

| Token | 值 | 用途 |
|---|---|---|
| `--font-ui` | `'DM Sans','Noto Sans SC',sans-serif` | 界面、按钮、输入、导航 |
| `--font-display` | `'Cormorant Garamond','Noto Sans SC',serif` | Hero 标题、区块标题、卡片标题 |
| `--font-reading` | `'Crimson Pro','Noto Sans SC',serif` | 卡片摘要、正文阅读 |

字重与字号由具体组件决定，标题使用较窄字距 `letter-spacing:-.02~-.045em`。

### 2.2 空间

| Token | 值 |
|---|---|
| `--space-xs` | 4px |
| `--space-sm` | 8px |
| `--space-md` | 16px |
| `--space-lg` | 24px |
| `--space-xl` | 32px |
| `--space-2xl` | 48px |
| `--space-3xl` | 64px |

### 2.3 圆角

| Token | 值 | 用途 |
|---|---|---|
| `--radius-sm` | 8px | 输入、小按钮 |
| `--radius-md` | 12px | 卡片、菜单 |
| `--radius-lg` | 16px | 大卡片、面板 |
| `--radius-xl` | 22px | 详情页顶部 |

### 2.4 动效

| Token | 值 |
|---|---|
| `--motion-fast` | 150ms |
| `--motion-base` | 220ms |
| `--motion-slow` | 420ms |
| `--ease-editorial` | `cubic-bezier(.2,.7,.2,1)` |

规则：

- 动效只在进入、悬停、状态反馈和详情转场中承担明确职责；
- 不延迟搜索、筛选、阅读或管理操作；
- 长列表 stagger 每项延迟 30–40ms，最多 10 项；
- 减少动效（应用内 `reduce-motion` 或系统 `prefers-reduced-motion`）下关闭持续运动和大幅转场，直接显示最终状态。

### 2.5 焦点

| Token | 值 |
|---|---|
| `--focus-ring` | `2px solid var(--user-accent)` |
| `--focus-offset` | 3px |

应用于所有 `button/input/select/textarea/summary` 的 `:focus-visible`。

### 2.6 东方博物馆材质色（语义参考）

| 色 | 值 | 用途 |
|---|---|---|
| 象牙 `--museum-ivory` | `#f5efe6` | 暖白主题背景 |
| 深木 `--museum-wood` | `#3b291f` | 深棕文字/边框 |
| 青玉 `--museum-jade` | `#8bd9cf` | 已验证、科技光 |
| 琥珀 `--museum-amber` | `#e8bd70` | 收藏、重点光 |
| 墨蓝 `--museum-ink` | `#0c1017` | 暗夜主题背景 |

主题变量（`--theme-bg/sidebar/surface/surface-strong/ink/muted/line/shadow`）与用户主色 `--user-accent` 由主题层运行时注入。

## 3. 主题

三套预设（本机 `localStorage` 持久化，不写入卡片数据库）：

1. 暗夜墨蓝 / 数字夜馆
2. 暖白纸张 / 东方明亮展厅
3. 琥珀暖色 / 铜色收藏展厅

每套主题必须保持同一套组件状态：默认、悬停、聚焦、禁用、加载、成功、错误。

## 4. 组件约束

### 4.1 卡片（内容图鉴 / 数字展品）

- 封面：类型专属抽象展品图（技能/知识/项目资料/AI 产出），`img loading="lazy"`，装饰性；
- 状态光：已验证用青玉 Border Beam，收藏用琥珀 Border Beam，普通卡片不持续闪烁；
- 重点展品（收藏卡片）：悬停 6° 内 3D 倾斜 + 封面高光；触屏/减少动效关闭；
- Spotlight 聚光跟随鼠标，不抢占收藏按钮；
- 摘要使用阅读字体，可扫描，不截断个人数据；
- 收藏按钮必须可独立点击（`stopPropagation`）。

### 4.2 详情页

- 从卡片进入可用原生 View Transitions，失败回退直接打开；
- 返回保留列表筛选/滚动上下文；
- 版本恢复前二次确认；
- 正文阅读时降低空间动效；
- 版本行使用 stagger 进入（40ms×index，上限 10）。

### 4.3 命令面板

- `Ctrl/Cmd + K` 打开，Esc/遮罩关闭；
- 命令项 stagger 进入（35ms×index，上限 10）；
- 键盘导航顺序与视觉顺序一致；
- 不影响普通关键词搜索。

### 4.4 设置页

- 本地数据安全、操作审计、导入状态三块；
- 审计行 stagger 进入（30ms×index，上限 10）；
- 备份/导入状态有成功/失败/进行中反馈；
- 不显示虚构业务数据。

## 5. 3D 与性能门禁

- Three/R3F 必须懒加载为独立 chunk，主包不直接引入 Three；
- 粒子使用 `Points + BufferGeometry`，不使用大量独立 Mesh；
- 当前粒子约 90 点，Drei `Sparkles` 约 40，`AdaptiveDpr` 自动降采样；
- 减少动效下粒子静止、Sparkles 为 0；
- 无 WebGL 时保留静态 Hero、CSS 光效和核心业务；
- 实验性 Shader 只放 `sketches/`，不进入正文/搜索结果/数据表格。

## 6. 响应式

| 断点 | 行为 |
|---|---|
| >900px | 三列卡片，侧边栏 242px |
| 651–900px | 两列卡片，侧边栏 220px |
| ≤650px | 侧边栏抽屉、单列卡片、顶部操作图标优先、关系表单单列、审计行换行 |

验收：无横向滚动；核心按钮可见；正文不被固定层遮挡；窄屏不强制启用 3D。

## 7. 可访问性

- 所有交互控件有可见 `:focus-visible`；
- 键盘可完成导航、搜索、命令、收藏、详情、关闭、恢复；
- `prefers-reduced-motion` 与应用内减少动效实时生效（含运行中变更）；
- Canvas 有语义描述（`role="img"` + `aria-label`），纯装饰层 `aria-hidden`；
- 无结果状态提供下一步；
- 状态不只依赖颜色。

## 8. 图标与文本

- 图标统一使用 `lucide-react`；
- 不使用 emoji 作为功能图标；
- 文案使用中文，清晰、无夸张装饰。

## 9. 数据与安全边界

- 真实个人数据只保存在 `D:\工作台数据`，不写入 GitHub；
- GitHub 只保留脱敏代码框架、文档模板、测试代码、构建配置和示例数据；
- 生图提示词只使用抽象产品语义，不含个人数据、卡片正文、文件、数据库、求职资料或凭据；
- `vendor/` 为只读研究源，不注入 Hermes 全局技能，不上传 GitHub；
- 不实现永久删除、关系删除或云同步。

## 10. 资源来源记录

| 资源 | 用途 | 许可/状态 |
|---|---|---|
| UI UX Pro Max（vendor/） | 设计系统检索参考 | MIT，本地只读 |
| React Bits / Magic UI / Aceternity | 组件与动效参考 | 按需改造，未整套安装 |
| `three` / `@react-three/fiber` / `@react-three/drei` | 展厅视觉层 | 已安装并懒加载 |
| `lucide-react` | 图标 | 已安装 |
| Cormorant Garamond / Crimson Pro / DM Sans / Noto Sans SC | 字体 | Google Fonts |
| `image_generate` | 抽象展品封面/主视觉 | 外部生图服务，仅抽象提示词 |

## 11. 变更方式

- 新增组件/动效/依赖前，回读 PRD 与图鉴；
- Token 变更必须同步更新本文件、`src/styles.css` 和图鉴；
- 实验性视觉先进入 `sketches/`，验证后再决定是否进入正式页面；
- 每次发布前完成构建、测试、运行态、数据边界检查。
