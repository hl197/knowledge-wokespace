# 前端组件、视觉资产与交互图鉴

> 项目：我的 AI 工作台
> 用途：为后续前端实现提供组件选型、视觉资产、生图提示词、动效和工程约束。
> 原则：不照搬品牌网站；只提炼公开组件和设计原则。真实个人数据不进入外部视觉服务。
> 关联文档：[FRONTEND-PRD.md](./FRONTEND-PRD.md)

---

## 1. 设计总纲

### 1.0 UI UX Pro Max 本地研究源

本项目已将 `nextlevelbuilder/ui-ux-pro-max-skill` 以只读研究源下载到：

```text
vendor/ui-ux-pro-max-skill/
```

使用范围：读取其 `data/*.csv`、BM25 搜索脚本与生成的设计系统；不将其完整 Agent Skill 注入 Hermes，不让它接触 `D:\工作台数据`，不自动执行其依赖安装或覆盖当前 PRD。

本轮实际采用的检索结论：

- Style：Editorial Grid / Magazine；强调非对称内容层级、展品式大图、滚动 reveal 与 parallax 装饰层。
- Typography：Cormorant Garamond + Crimson Pro 的 academia/library 气质；已应用到 Hero 标题、区块标题与卡片摘要。
- Motion：300–450ms stagger、8–16px reveal、装饰层小幅 parallax；长列表不使用过度 overshoot。
- UX：focus ring、`prefers-reduced-motion`、无结果下一步、Canvas 语义标签和 Three.js 粒子数量门禁。
- Three.js：使用懒加载、`Points + BufferGeometry`，不使用大量独立 Mesh；当前粒子层保持 90 点并提供静态降级。

仓库默认生成的紫粉 Swiss/Minimalism 方案未直接采用，因为它与已确认的东方数字图书馆色彩世界观冲突；本项目只吸收可验证的排版、动效、UX 与性能规则。

### 1.1 产品表面类型

| 类型 | 工作台中的页面 | 优先事项 |
|---|---|---|
| Explore | 总览、全部内容、技能、知识、项目资料、AI 产出 | 浏览、分类、视觉识别、筛选、探索节奏 |
| Command / Inspect | 搜索、命令面板、详情、版本、关系、回收站 | 速度、键盘操作、清晰状态、阅读和操作 |
| Configure | 主题、动效、主色、后续设置页 | 明确选项、即时预览、本地保存 |

工作台不是营销落地页。首屏主视觉是“数字博物馆入口”，目标是强化探索氛围，不能遮挡搜索、卡片和操作。

### 1.2 世界观

> 明亮现代东方数字图书馆 + 数字博物馆展陈。

关键词：宋式雅集、象牙白、深棕木材、茶色、青玉、青绿色数字光、星图、展柜、抽象展品、留白、知识收藏。

### 1.3 反模板化原则

避免：

- 默认紫色渐变铺满页面；
- 所有区块同权重、同尺寸、同样式；
- 无意义的大数字、图标顶置卡片、装饰性左边框；
- 背景动效遮挡内容；
- 把真实个人内容送到生图服务；
- 复制任何网站的专有布局、品牌文字或具体视觉。

---

## 2. 主题与配色系统

主题是三种“展厅材质”，不是简单的暗/亮色切换。所有主题共享信息层级和组件结构，但使用不同的表面、光线、阴影和主色。

### 2.1 暗夜墨蓝：数字夜馆

| 维度 | 规则 |
|---|---|
| 气质 | 夜间展厅、深蓝空间、青玉冷光、少量琥珀展灯 |
| 背景 | 墨蓝黑、深靛、低对比石材感 |
| 主色 | 青玉绿、冰蓝或用户自选冷色 |
| 强调色 | 琥珀金用于收藏、已验证、重点展品 |
| 适用 | 夜间浏览、沉浸探索、粒子和光柱效果 |
| 动效 | 可使用较明显的粒子、轨道、柔和光柱 |
| 风险 | 不得压暗正文；正文层必须保持高对比 |

建议 Token：

```css
--museum-bg: #101827;
--museum-surface: #172235;
--museum-ink: #eef3fb;
--museum-muted: #91a1b8;
--museum-jade: #83c9bd;
--museum-gold: #d7a45c;
```

### 2.2 暖白纸张：东方明亮展厅

| 维度 | 规则 |
|---|---|
| 气质 | 象牙白展馆、宋式留白、纸张与石材、日光展陈 |
| 背景 | 象牙白、米色、浅石材 |
| 主色 | 茶棕、青玉、暖铜或用户自选主色 |
| 强调色 | 青绿色数字光、低饱和琥珀 |
| 适用 | 长时间阅读、内容浏览、主视觉和插画展示 |
| 动效 | 光线、微尘、纸张层次、缓慢展品浮动 |
| 风险 | 控制浅色表面对比度，不能只靠颜色表达状态 |

建议 Token：

```css
--museum-bg: #f5efe6;
--museum-surface: #fffaf3;
--museum-ink: #2b2925;
--museum-muted: #867b70;
--museum-jade: #5f9e92;
--museum-gold: #bd7d45;
```

### 2.3 琥珀暖色：铜色收藏展厅

| 维度 | 规则 |
|---|---|
| 气质 | 铜、琥珀、茶色木材、收藏柜、晚间暖灯 |
| 背景 | 深茶、铜棕、暖黑 |
| 主色 | 琥珀金、赤铜、浅青玉点缀 |
| 强调色 | 珊瑚或青绿色只作为少量科技信号 |
| 适用 | 项目资料、收藏、归档、展品陈列 |
| 动效 | 光柱、细小火花、金属反射、慢速边缘光 |
| 风险 | 避免变成过度复古或酒吧风 |

建议 Token：

```css
--museum-bg: #24180f;
--museum-surface: #352518;
--museum-ink: #f5e7d0;
--museum-muted: #bda68a;
--museum-gold: #e6a15d;
--museum-jade: #8fcab5;
```

### 2.4 用户主色

用户可在预设基础上选择主色。主色应用范围：

- 当前导航指示；
- 主按钮；
- 卡片聚光；
- 主题菜单选中态；
- 命令面板焦点；
- 局部光束和粒子色。

主色不应覆盖：正文颜色、危险状态颜色、成功状态颜色、来源/状态文字的对比度。

---

## 3. 外部灵感来源如何转化

### 3.1 MotionSites：空间叙事与动态节奏

来自用户提供的 Interactive Discovery、Data Storytelling、Dreamcore Landing 方向。

可提炼原则：

| 原则 | 工作台转化 | 禁止做法 |
|---|---|---|
| 视觉入口先建立情绪 | 首屏使用东方数字图书馆主视觉、灯光、粒子和展品 | 不把工作台变成只有 Hero 的营销页 |
| 滚动/转场讲故事 | 卡片进入详情时强化“展品 → 检查器”连续性 | 不让动画阻塞阅读或操作 |
| 3D 作为叙事焦点 | Hero 中央数字展品、关系空间候选 | 不把每张卡片都做成重型 3D |
| 多层景深 | 主视觉背景、粒子、内容层、顶部栏分层 | 不让层级遮挡搜索和文字 |
| 惊艳但可控 | 首屏和关键交互有强效果，正文阅读自动安静 | 连续炫技循环动画 |

可借鉴的页面节奏：

```text
进入：展厅光线/主视觉出现
→ 探索：内容图鉴和类型展品
→ 检查：全屏详情、版本和关系
→ 返回：保留筛选和浏览上下文
```

### 3.2 Land-book：编辑精选和内容层级

Land-book 是网站设计灵感画廊，适合吸收：

- 首屏主视觉与内容列表的明确分层；
- 卡片瀑布/图鉴的视觉节奏；
- 大图与文字的比例；
- 颜色、留白和内容优先级；
- 不同网站风格的横向参考。

不应直接复制任何具体网站的页面结构、品牌文本或视觉资产。

### 3.3 React Bits：互动组件和背景

React Bits 是高星开源 React 动效组件库。当前项目采用“按需改造具体效果”的方式，不整包安装。

### 3.4 Magic UI：精致状态与边缘动效

Magic UI 的价值在于：

- 聚光卡片；
- Border Beam；
- Blur Fade；
- Animated List；
- Light Rays；
- 命令/终端/通知列表等微交互。

当前项目不使用 Tailwind，因此优先提炼组件行为并用 React/CSS 实现，避免为一个效果迁移整个样式体系。

### 3.5 Aceternity UI：强交互与空间感

适合借鉴：

- Card Spotlight；
- 3D Card；
- Moving Border；
- Lamp / Spotlight；
- Hover Border Gradient；
- Text Reveal Card；
- Parallax Scroll。

多数组件依赖 Tailwind 和 `motion`，接入前必须评估是否值得安装 `framer-motion` 或是否采用无依赖 CSS/React 改造。

---

## 4. 组件图鉴

状态说明：

- `已接入`：当前代码已经使用。
- `规划`：PRD 已确认，但尚未实现。
- `候选`：值得实验，需要用户确认或技术评估。

### 4.1 东方数字图书馆 Hero

| 项目 | 说明 |
|---|---|
| 状态 | 已接入基础版 |
| 灵感 | MotionSites 叙事首屏 + React Bits Light Pillar / Particles 方向 |
| 位置 | 总览页顶部 |
| 目的 | 建立“数字博物馆 + 东方未来图书馆”的世界观，不取代资产浏览 |
| 视觉 | 明亮象牙白、深棕书架、茶色、青玉光、星图、中央数字艺术装置 |
| 布局 | 左侧标题与简介；右侧/背景为主视觉；下方紧接搜索与内容图鉴 |
| 交互 | 首次进入淡入；主题切换时改变灯光和粒子色；减少动效时静止/降低透明度 |
| 技术 | WebP 主视觉 + CSS 遮罩 + 原生 Canvas 2D 粒子/光柱/网格层 |
| 依赖 | 不依赖 Three/R3F；Canvas 层按需加载 |
| 降级 | WebGL 不可用时仍显示静态图片 + CSS 光晕；减少动效时停止旋转 |
| 验收 | 主标题可读；搜索可操作；背景不遮挡按钮；主题变化明显但不跳闪 |

#### 主视觉配图提示词

```text
Premium cinematic hero illustration for an original personal AI knowledge workspace.
A bright modern Eastern digital library and digital museum, inspired by Song-dynasty elegance through abstract materials only: ivory walls, deep brown wooden library shelves, tea-colored wood, subtle lattice shadows, jade-green digital light, circular star-map traces, central sculptural knowledge artifact, warm natural museum light, refined contemporary 3D concept art, generous negative space for interface overlay. No people, no readable text, no logos, no UI, no letters, no numbers, no personal data.
```

#### 主题变体提示词

| 主题 | 追加提示词 |
|---|---|
| 暗夜墨蓝 | `midnight indigo gallery, cool jade and ice-blue light, small amber display lamps, deep but readable` |
| 暖白纸张 | `bright ivory paper gallery, warm daylight, pale stone, tea-brown wood, quiet jade accents` |
| 琥珀暖色 | `copper and amber collector gallery, tea-brown wood, warm bronze reflections, restrained jade highlights` |

---

### 4.2 React Bits 风格 Spotlight Card

| 项目 | 说明 |
|---|---|
| 状态 | 已接入改造版 |
| 官方组件 | React Bits Spotlight Card |
| 官方关键 Props | `spotlightColor`、`className` |
| 位置 | 内容图鉴卡片 |
| 目的 | 鼠标靠近时让卡片像被展厅聚光灯照亮，强调“展品”感 |
| 配色 | 聚光色来自卡片类型或用户主色；透明度低，不能遮挡标题 |
| 布局 | 聚光层在卡片内容下方，封面/标题/操作始终在上方 |
| 技术 | 原生 React `onMouseMove` + CSS radial-gradient；无需新增依赖 |
| 性能 | 只更新当前 hover 卡片的 CSS 变量；不使用全局监听 |
| 降级 | 触屏和减少动效下显示静态弱光或不显示聚光 |
| 验收 | 收藏按钮独立可点击；聚光不导致卡片跳动；键盘 focus 仍可见 |

#### 展品卡片封面提示词

```text
High-end 3D concept-art museum exhibit cover for a personal knowledge asset. Abstract object only, no people, no text, no logo, no UI. Bright refined gallery lighting, premium materials, centered artifact with room around it for card labels, original digital museum aesthetic.
```

---

### 4.3 React Bits Tilted Card / Aceternity 3D Card

| 项目 | 说明 |
|---|---|
| 状态 | 候选，未接入 |
| 官方关键 Props | `rotateAmplitude` 默认 14、`scaleOnHover` 默认 1.1、可选 overlay content |
| 位置 | 仅限总览中的重点卡片、收藏展品或精选技能；不应用到所有列表卡片 |
| 目的 | 增加“可拿起展品”的深度感 |
| 配色 | 保持主题展厅色；避免彩虹反射 |
| 布局 | 卡片容器保留固定尺寸，内部视觉封面产生 3D 倾斜，正文不发生过度变形 |
| 技术选择 A | 使用 React Bits 原组件，需要 `motion` 依赖 |
| 技术选择 B | 自写 CSS transform + pointer 变量，依赖更少但物理感较弱 |
| 性能 | 单次最多 1–3 张重点卡片启用；禁用移动端 Tilt；减少动效关闭 |
| 风险 | 过强倾斜影响文字阅读、点击和可访问性 |
| 验收 | 倾斜角建议 4–8°，缩放建议 1.02–1.04；正文保持可读；触屏不误触 |

#### 重点展品封面提示词

```text
Premium collectible digital museum exhibit, a single abstract knowledge artifact floating above a small pedestal, translucent jade glass, brushed bronze, ivory ceramic, warm gallery spotlight, cinematic depth, centered composition, no text, no people, no logo, no UI, no personal data.
```

---

### 4.4 Magic UI Border Beam / Aceternity Moving Border

| 项目 | 说明 |
|---|---|
| 状态 | 规划 |
| 官方作用 | 沿容器边缘循环的光束；Magic UI 可控制 `duration`、`size`、`delay`、`colorFrom`、`colorTo`、`borderWidth` |
| 位置 | 当前选中卡片、重点收藏、导入完成、正在生成的 AI 产出；不用于所有卡片 |
| 目的 | 提示“当前被关注的展品”或“状态正在变化” |
| 配色 | 暗夜主题：冰蓝 → 青玉；暖白：茶铜 → 青玉；琥珀：琥珀 → 金色 |
| 布局 | 光束只沿外边框行走，不覆盖正文和按钮 |
| 技术选择 A | Magic UI copy-paste，通常涉及 Motion/Tailwind |
| 技术选择 B | CSS `conic-gradient` + mask / pseudo-element，当前项目更适合 |
| 性能 | 只显示在选中/重点项；动画周期 5–8 秒；减少动效停止 |
| 验收 | 轮廓不闪烁，不影响卡片点击，主题切换色彩同步 |

#### 重点状态说明

```text
已验证：低频青玉边缘光
收藏：琥珀金边缘光
新导入待验证：柔和铜色边缘光
正在处理：短周期青绿流光
```

---

### 4.5 React Bits Light Pillar

| 项目 | 说明 |
|---|---|
| 状态 | 候选，首屏粒子已接入，光柱待实验 |
| 官方关键 Props | `topColor`、`bottomColor`、`intensity`、`rotationSpeed`、`glowAmount`、`pillarWidth`、`pillarHeight`、`noiseIntensity`、`interactive`、`quality` |
| 位置 | Hero 中央展品后方、详情页重大版本恢复成功状态、未来关系空间入口 |
| 目的 | 像博物馆顶灯或数字展品光柱，强化空间深度 |
| 配色 | 暗夜：冰蓝 → 青玉；暖白：香槟金 → 青玉；琥珀：铜金 → 蜜金 |
| 布局 | 只占 Hero 的视觉右侧或中央，不遮挡左侧标题和搜索 |
| 技术 | 可以继续使用当前 Three/R3F；不需要安装完整 React Bits |
| 性能 | 质量分档：桌面高、窄窗口中、减少动效/无 WebGL 静态 CSS 光柱 |
| 风险 | 过亮、过快、与主视觉冲突；WebGL 可能增加主包 |
| 验收 | 文字可读，粒子/光柱不超过视觉层，切换主题不跳帧 |

#### 光柱氛围提示词（用于生图或设计参考）

```text
Elegant vertical museum light pillars around an abstract jade knowledge sculpture, warm ivory gallery, subtle teal holographic glow, tea-brown wood, refined dust particles, soft cinematic lighting, original digital museum atmosphere, no text, no people, no UI.
```

---

### 4.6 React Bits Particles / 自定义 MuseumParticles

| 项目 | 说明 |
|---|---|
| 状态 | 已接入基础版 |
| 位置 | 总览 Hero 背景 |
| 目的 | 模拟展厅微尘、星图节点和数字知识碎片 |
| 配色 | 随主题切换：夜馆冰蓝、暖白茶铜、琥珀金色 |
| 技术 | `three` + `@react-three/fiber` + `Points` |
| 交互 | 当前缓慢旋转；后续可有限度鼠标视差，不追踪用户数据 |
| 降级 | `prefers-reduced-motion` 与应用内减少动效时静止；WebGL 失败时仅显示主视觉 |
| 验收 | 不遮挡点击区域；粒子数量可配置；详情打开时暂停或卸载 |

建议参数：

```text
粒子数量：60–140
尺寸：0.02–0.05
透明度：0.35–0.80
旋转速度：0.01–0.04 rad/s
```

---

### 4.7 Magic UI Animated List

| 项目 | 说明 |
|---|---|
| 状态 | 规划 |
| 官方用途 | 列表项目依次延迟出现，适合通知或事件列表 |
| 位置 | 命令面板、版本历史、审计日志、最近更新 |
| 特色 | 项目按顺序出现，支持列表节奏和视觉层级 |
| 配色 | 列表图标/状态色采用主题 Token；不要使用 emoji 作为最终图标体系 |
| 布局 | 命令面板列表每项左侧图标、中间标题/说明、右侧类型/快捷键 |
| 技术选择 A | Magic UI 组件，可能依赖 Motion/Tailwind |
| 技术选择 B | CSS animation-delay + `prefers-reduced-motion`，更适合当前 Vite/CSS 架构 |
| 性能 | 大列表只对首批可见项动画；筛选后不重复大规模入场 |
| 验收 | 箭头/Tab/Enter 键盘导航不受动画影响；减少动效立即显示 |

---

### 4.8 Magic UI Blur Fade

| 项目 | 说明 |
|---|---|
| 状态 | 候选 |
| 官方 Props | `duration` 默认 0.4 秒、`delay`、`offset` 默认 6、`direction`、`inView` |
| 位置 | Hero 标题、内容图鉴首批卡片、空状态、详情区块进入 |
| 目的 | 把大面积页面切换变成有层次的淡入，不需要复杂 3D |
| 配色 | 不直接改变颜色，只配合主题表面色 |
| 技术 | 可用 CSS `opacity` + `transform` + staggered delay 实现，无需 Motion |
| 风险 | 如果每次筛选都重放，会干扰效率 |
| 验收 | 初次进入可见；快速搜索时不闪烁；减少动效时无位移 |

---

### 4.9 命令面板

| 项目 | 说明 |
|---|---|
| 状态 | 已接入基础版，待完善 |
| 灵感 | 编辑器控制台、Magic UI Terminal / Command Palette、Raycast 类命令体验 |
| 位置 | 全局 `Ctrl/Cmd + K`，搜索框聚焦 |
| 特色 | 在视觉探索页面中提供高效率键盘入口 |
| 配色 | 高于页面表面的独立展柜；主色作为焦点和选中态，不大面积铺色 |
| 布局 | 顶部输入框；下方分组命令；每项标题、说明、类型、快捷键 |
| 当前功能 | 打开全部内容、技能、知识、主题、新建、刷新 |
| 后续功能 | 搜索卡片、按类型/标签/状态筛选、最近操作、键盘上下选择、Enter 执行 |
| 技术 | React state + 原生键盘事件；后续可按需引入 Radix Dialog 或 `cmdk` |
| 验收 | Esc 关闭；焦点自动进入输入框；命令列表可键盘操作；不影响真实搜索 |

---

### 4.10 关系空间与知识星图

| 项目 | 说明 |
|---|---|
| 状态 | 长期候选，不属于当前 P0 |
| 灵感 | MotionSites Spatial Mapping / Data Storytelling、R3F 场景 |
| 位置 | 单独的“关系空间”或项目详情，不替代列表页 |
| 视觉 | 环形书架、星图轨道、青玉节点、展品连线 |
| 数据 | 仅读取已有 `card_relations`，不把个人正文暴露给外部图形服务 |
| 技术 | Canvas 2D 关系星图（按需加载）+ 列表/详情文本替代 |
| 风险 | 视觉很强但信息检索效率低；必须保留列表/详情替代入口 |
| 验收 | 节点可点击、可返回卡片详情、无 WebGL 时回退到关系列表 |

---

## 5. 卡片类型视觉系统

每种卡片类型拥有独立展品语言，但共享“东方数字博物馆”的材质和光线。

| 类型 | 展品意象 | 颜色 | 材质 | 页面用途 |
|---|---|---|---|---|
| 技能 | 青玉工具、精密模块、悬浮符号 | 青玉、银、淡紫 | 玉石、玻璃、拉丝金属 | 技能卡、精选技能、自动化能力 |
| 知识 | 水晶书页、玻璃档案、知识棱镜 | 冰蓝、青绿、象牙白 | 玻璃、纸张、薄雾 | 知识卡、阅读、资料沉淀 |
| 项目资料 | 木构模型、工程展台、地图结构 | 茶棕、铜、琥珀 | 木材、青铜、石材 | 项目、PRD、技术资料 |
| AI 产出 | 生成光体、粒子花束、能量流 | 青绿、金、珊瑚点缀 | 发光玻璃、光纤、薄膜 | AI 输出、方案、生成物 |
| 用户画像/偏好 | 印章、折叠纸片、温润玉佩 | 浅粉、茶色、金 | 纸张、玉、陶瓷 | 默认弱展示，强调隐私边界 |

### 各类型生图提示词

#### 技能

```text
A premium 3D digital museum exhibit representing an AI skill: abstract jade-green precision tool, translucent glass modules, brushed silver details, subtle lavender signal light, displayed on an ivory and dark wood pedestal, warm gallery lighting, original contemporary Eastern digital library aesthetic, no text, no people, no logo, no UI, no personal data.
```

#### 知识

```text
A premium 3D museum exhibit representing knowledge: floating translucent glass book pages, a crystalline archive prism, pale ivory paper textures, ice-blue and jade light, subtle star-map orbit, warm modern library gallery, original artistic composition, no text, no people, no logo, no UI, no personal data.
```

#### 项目资料

```text
A premium 3D museum exhibit representing a project: an abstract architectural model with dark tea-brown wood, bronze engineering joints, amber map lines and a jade accent, displayed in a bright ivory digital library gallery, sophisticated contemporary museum art, no text, no people, no logo, no UI, no personal data.
```

#### AI 产出

```text
A premium 3D museum exhibit representing AI-generated work: a luminous abstract energy sculpture made of flowing jade light, glass petals, fine gold particles and subtle coral reflections, displayed in a warm ivory digital museum, cinematic but clean composition, no text, no people, no logo, no UI, no personal data.
```

#### 空状态

```text
A serene empty display case in a bright Eastern digital library museum, ivory stone, tea-brown wood, a small jade glow waiting at the center, subtle dust particles and star-map lines, welcoming sense of beginning and discovery, no text, no people, no logo, no UI, no personal data.
```

---

## 6. 页面布局规范

### 6.1 总览

```text
顶部应用栏
├── 面包屑
├── 本机安全状态
├── 主题入口
├── 导入
└── 新建卡片

东方数字图书馆 Hero
├── 主视觉与粒子/光柱层
├── 叙事标题和简述
└── 展厅入口感，不抢夺操作焦点

搜索与筛选
├── 全局搜索 / 命令入口
├── 标签
├── 状态
└── 结果数量

资产图鉴
├── 可浏览的展品卡片
├── 重点资产可使用 Tilted / Border Beam
└── 正常资产使用 Spotlight Card
```

### 6.2 详情

```text
返回资产图鉴 + 状态
├── 类型、标题、摘要、收藏、更新时间
├── 来源、可见范围、原始路径
├── 正文阅读区
├── 标签
├── 版本历史
├── 关联卡片
└── 回收站/归档等生命周期操作
```

### 6.3 命令面板

```text
输入框
├── 直接命令
├── 卡片搜索结果
├── 筛选建议
├── 最近动作
└── 快捷键提示
```

---

## 7. 动效时间轴

| 场景 | 效果 | 推荐时长 | 减少动效行为 |
|---|---|---:|---|
| 首次进入 | Hero 光线、主视觉、粒子渐入 | 500–1200ms | 直接显示静态状态 |
| Hero 粒子 | 缓慢旋转/漂浮 | 持续 | 静止或隐藏 |
| 卡片 hover | Spotlight、上移、低幅边缘光 | 150–280ms | 保留 focus，不位移 |
| 筛选结果 | Blur Fade / staggered reveal | 120–350ms | 直接切换 |
| 详情打开 | 卡片到全屏检查器 | 220–420ms | 直接淡入 |
| 主题切换 | 表面、灯光、粒子颜色过渡 | 250–500ms | 直接切换颜色 |
| 版本恢复成功 | 展品边缘光 + 成功提示 | 400–800ms | 只显示提示 |

禁止：自动播放音频、持续高频闪烁、无法跳过的长转场、阅读中不断漂浮的大物体。

---

## 8. 工程决策表

| 决策 | 当前结论 | 原因 |
|---|---|---|
| React Bits | 选择性改造 | 适合 Spotlight、Particles、Light Pillar，但不整包安装 |
| Magic UI | 选择性改造 | Border Beam、Animated List、Blur Fade 值得使用，但当前项目非 Tailwind |
| Aceternity | 选择性改造 | 3D Card、Moving Border、Spotlight 有参考价值，通常需要 Motion/Tailwind |
| Motion/Framer Motion | 暂未安装 | Tilted Card、Animated List 等需要时单独确认，记录包体影响 |
| Three/R3F | 已安装 | 已用于粒子背景；后续基于真实体验决定是否进一步扩展 |
| Drei | 暂未安装 | 只有做复杂 3D 展品/关系空间时评估 |
| 图片资源 | 项目内保存抽象视觉资产 | 视觉可打包、可离线，且不包含个人数据 |
| 真实数据 | 永远不用于外部生图 | 隐私和本地优先边界 |

---

## 9. 视觉与工程验收清单

### 视觉

- [ ] 主题切换后像不同材质的展厅，而不是简单换背景色。
- [ ] 首屏明确传达东方数字图书馆与数字博物馆。
- [ ] 卡片像可收藏展品，但仍能快速扫描标题和摘要。
- [ ] 插画/背景明亮、温暖、科技和艺术平衡。
- [ ] 不出现文字乱码、Logo、真人或个人内容的生图结果。

### 交互

- [ ] Spotlight Card 不抢占收藏按钮点击。
- [ ] 重点卡片的 Tilt/Border Beam 不影响正文阅读。
- [ ] 命令面板支持 Esc、方向键、Enter 和焦点管理。
- [ ] 详情返回保留列表筛选和滚动位置。
- [ ] 版本恢复有二次确认。

### 性能与降级

- [ ] 无 WebGL 时主视觉和核心卡片仍然可用。
- [ ] 减少动效时所有持续动画停止或静止。
- [ ] 3D 背景不阻塞搜索、筛选、详情和输入。
- [ ] 实测主包、首屏、GPU 使用和内存后再决定是否懒加载。
- [ ] 小窗口和窄屏可浏览卡片、打开详情和使用命令。

### 安全

- [ ] 视觉资产不包含真实个人数据。
- [ ] GitHub 只保存脱敏代码、模板、示例资源；不上传 `D:\工作台数据`。
- [ ] 所有外部组件来源和许可证都已记录。

---

## 10. 后续选型顺序

1. 完成 Hero 光柱和粒子运行态验证。
2. 确认是否把 Light Pillar 正式加入 Hero。
3. 对 1–3 张重点卡片实验 Tilted Card。
4. 对“已验证/收藏/正在处理”状态实验 Border Beam。
5. 用 Animated List 或 CSS stagger 完善命令面板和版本历史。
6. 生成四类展品封面候选并选择最终资产。
7. 最后做关系空间概念草图；不要跳过日常列表/详情效率。
