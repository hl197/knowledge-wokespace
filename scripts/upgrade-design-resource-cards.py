#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""将前端设计资源升级为跨项目、可供 Agent 调取的 Skill-style 知识卡片。"""
import json
import time
import urllib.request

API = "http://127.0.0.1:37821/api/cards"

RESOURCES = [
    {
        "title": "设计资源：Originkit 动画组件库",
        "old_id": "card-1787636142",
        "source": "https://www.originkit.dev/",
        "summary": "通用动画组件资源库：先读 description 判断需要粒子、动态背景、文字或图片动效，再按分类回到官方源页获取具体组件；适用于 React、Next.js、Vite 和 Framer。",
        "tags": ["设计资源", "前端", "动效", "动画组件", "粒子", "资源Skill"],
        "content": """# Originkit 动画组件库

description: 面向现代 Web 项目的动画组件注册表，覆盖粒子、动态背景、文字、图片画廊和按钮动效；用于 Agent 在前端项目中按需检索可复用视觉效果。

## When to use

- Hero 需要粒子、动态背景、光点或装饰性流场；
- 空状态、等待状态或 CTA 需要轻量动态反馈；
- 图片画廊需要 Coverflow、Swipe Stack 或空间排列；
- 短标题需要文字变形、模糊、粒子化或逐字出现；
- 原型阶段需要快速比较不同动效方向。

不用于：正文阅读、密集数据表格、持续高耗电动画、未经评估的核心导航动画。

## Source / license

- 官网：https://www.originkit.dev/
- Registry/API：https://mcp.originkit.dev/v1
- npm CLI：`@originkit/cli`
- npm 包公开信息标注 MIT；具体组件使用前回到官方组件源页确认许可证、依赖和更新时间。
- 远程获取组件可能需要登录、配额或 API Key；不得把凭据写入项目或提示词。

## Component catalog

### Interactive Elements

- Black Hole：中心吸附或空间入口氛围；
- Fluid Trail：指针跟随流体轨迹，触屏应关闭；
- SVG Particles：轻量粒子和 SVG 降级；
- Kinetic Grid：动态网格和背景结构。

### Image Gallery

- Spiral Images、Coverflow Carousel、Swipe Stack、Infinity Canvas：用于专题画廊，不替代数据列表。

### Text / Animation

- Smoky Text、Text Morph、Scramble Text、Weight Hover；
- Pixel Reveal、Star Burst、Particle Tunnel、Glitter Wrap；
- Snow Fall、Blinking Squares、Character Waves；
- Link Preview 等短时 CTA 反馈。

## Agent reading workflow

1. 先读取本卡 `description` 和 `When to use`；
2. 判断资源是否符合当前项目的界面用途、视觉方向和技术栈；
3. 只有匹配后，才打开官网分类/组件源页；
4. 从官方源页读取具体组件代码、许可证、依赖、浏览器限制和安装方式；
5. 优先复制少量源码或改写为原生 CSS/SVG/Canvas；
6. 完成构建、键盘、触屏、reduced-motion 和性能验证。

## Selection rules

- 每个页面最多选择 1–2 个主动画；
- 粒子、背景和文字动画不能同时高强度持续运行；
- 优先 CSS/SVG/Canvas，只有明确视觉收益时使用 WebGL；
- 指针跟随效果必须提供触屏和键盘降级；
- 动画不得遮挡主要内容或改变核心操作顺序。

## Prompt templates

- “从 Originkit 官方资源中检索适合这个页面 Hero 的低功耗粒子/动态背景组件，要求给出源页、许可证、依赖、移动端降级和 reduced-motion 方案。”
- “从 Originkit 中筛选适合短标题的文字动效，排除长正文逐字动画和持续高频效果。”
- “比较 Originkit 的粒子组件与当前技术栈，先给出是否值得接入的评估，不要直接安装。”

## Performance and accessibility

- 只对 transform/opacity/Canvas 绘制做动画；
- 控制粒子数量、帧率、DPR 和主线程工作；
- 支持 `prefers-reduced-motion`，直接渲染最终状态；
- 保持语义 DOM、可见 focus 和键盘操作；
- 不把真实业务数据、个人信息或私密文件放进视觉组件。

## Verification

记录具体官方组件 URL、许可证、复制范围、依赖、包体、降级和测试结果；不能只根据演示截图判断组件适用性。
""",
    },
    {
        "title": "设计资源：React Bits 动画组件库",
        "old_id": "card-1787636181",
        "source": "https://www.reactbits.dev/",
        "summary": "通用 React 动画与交互组件库：先读 description 和分类，再回到具体组件源页获取 Spotlight、Tilt、Particles、文字动画、列表和按钮源码；适用于按需复制改造。",
        "tags": ["设计资源", "前端", "动效", "React", "交互组件", "资源Skill"],
        "content": """# React Bits 动画组件库

description: 面向 React 的开源动画与交互组件集合，覆盖文字、背景、卡片、按钮、列表和光效；适合 Agent 按页面用途检索组件并复制少量源码进行项目化改造。

## When to use

- 需要 Spotlight、Tilt、Glare、Border 或磁性按钮；
- 需要 Hero 标题 reveal、blur、split 或逐字出现；
- 需要列表、Tabs、按钮和背景的轻量动效；
- 需要先制作可点击草图，再快速验证视觉方向。

不用于：整套替换项目设计系统、长正文逐字动画、所有卡片同时倾斜或未评估的重型背景。

## Source / license

- 官网：https://www.reactbits.dev/
- GitHub：https://github.com/DavidHDev/react-bits
- 组件许可和仓库许可以当前官方源页/源仓库为准；当前研究记录为 MIT + Commons Clause，复制前复核具体文件。
- 不直接安装完整库；复制单个组件时保留来源和许可说明。

## Component catalog

### Cards / interaction

- Spotlight Card：鼠标聚光；
- Tilted Card：重点卡片轻微倾斜；
- Glare Hover：封面高光；
- Border/Electric Border：状态边框光。

### Text

- Blur In、Gradual Spacing、Split/Character Reveal；只适合短标题。

### Background / particles

- Particles、Light Pillar、Aurora、Gradient；需评估 GPU、移动端和 reduced-motion。

### Content / UI

- Animated List、Tabs、Magnetic Button、Scroll Expand、Gradual Blur。

## Agent reading workflow

1. 先读取本卡 `description`、`When to use` 和 `Component catalog`；
2. 判断组件是否匹配当前项目的界面类型、内容密度、技术栈和视觉方向；
3. 再打开 reactbits.dev 的具体组件源页；
4. 读取源码、安装说明、依赖、许可证、浏览器限制和示例；
5. 复制到项目组件目录或改写为现有 CSS/React；
6. 验证默认、hover、focus、disabled、触屏和 reduced-motion 状态。

## Selection rules

- 每个视图最多 1 个持续氛围效果 + 1 个局部交互效果；
- 长列表只做短延迟 stagger，不使用逐项长时间等待；
- 文字动画只用于短标题，不处理正文；
- 重点卡片数量控制在 1–3 个；
- 收藏、提交、关闭等主要按钮必须独立可操作。

## Prompt templates

- “从 React Bits 官方源页筛选适合这个页面 Hero 的文字入场和低频背景，给出源码、许可证、依赖和 reduced-motion 方案。”
- “选择 Spotlight + Glare 的卡片方案，要求收藏按钮独立可点、触屏关闭 Tilt、不能改变布局。”
- “检索 Animated List，设计 30–40ms stagger，保证键盘导航顺序不变。”

## Verification

必须记录具体组件源页、许可证、依赖、复制范围和性能结果；通过构建、键盘、触屏、窄屏、reduced-motion 和真实主流程验证。
""",
    },
    {
        "title": "设计资源：Aceternity UI",
        "old_id": "card-1787636316",
        "source": "https://ui.aceternity.com/",
        "summary": "通用 React/Next.js 高视觉组件参考：先判断是否需要 Spotlight、3D Card、Moving Border 或滚动区块，再回到官方源页读取具体代码；基础组件与 Pro 模板分开评估。",
        "tags": ["设计资源", "前端", "React", "组件库", "动效", "资源Skill"],
        "content": """# Aceternity UI

description: 面向 React/Next.js 的高视觉质量组件和页面区块资源，擅长 Spotlight、3D Card、Moving Border、背景光效与滚动叙事；适合专题页、展厅、品牌页和重点内容，不应直接替代效率型应用布局。

## When to use

- 页面需要重点展品、Hero 或专题区块的空间感；
- 需要 Card Spotlight、3D Card、Moving Border、光束或滚动 reveal；
- 需要快速比较高端视觉方向，再改写为项目设计系统。

不用于：整页照搬、所有卡片 3D 化、正文背景持续运动或未经核对的 Pro 模板。

## Source / license

- 官网：https://ui.aceternity.com/
- 基础组件免费，完整模板/Pro 内容单独计费；具体组件许可与依赖必须逐项回到官方源页确认。
- 默认只提炼效果和结构，复制少量免费组件，不采购或复制 Pro 模板。

## Component catalog

- Card Spotlight / Spotlight：局部聚光；
- 3D Card：重点内容层次；
- Moving Border / Border：状态边框光；
- Animated Grid / Light Rays：Hero 或实验背景；
- Sticky Scroll Reveal：专题页滚动叙事。

## Agent reading workflow

1. 先读 description，确认页面是 Explore、Inspect、Landing 还是 Configure；
2. 判断效果是否服务用户操作，而不是只增加装饰；
3. 回到 ui.aceternity.com 的具体组件源页读取代码、许可证、依赖和安装步骤；
4. 只复制需要的组件，并改写为项目 Token；
5. 通过键盘、窄屏、触屏、reduced-motion 和构建验证。

## Selection rules

- 重点展品优先 Spotlight + 轻 Tilt，避免多个 3D 效果叠加；
- Moving Border 只用于有限状态；
- Sticky/Pin 只放独立专题或实验页，不劫持主工作台滚动；
- Pro 模板只作为设计研究，不作为默认依赖。

## Prompt templates

- “从 Aceternity UI 官方源页选择一个可按需复制的 Card Spotlight，适配当前项目 Token，保留键盘和 reduced-motion。”
- “检索 Moving Border/Light Rays，评估是否适合状态反馈，要求说明持续动画成本和静态降级。”

## Verification

记录具体源页、许可证、依赖、复制范围、包体、移动端行为和降级方式；不能只引用组件截图。
""",
    },
    {
        "title": "设计资源：Three.js 3D 核心库",
        "old_id": "card-1787636317",
        "source": "https://threejs.org/",
        "summary": "通用 3D/WebGL 核心库：先读 description 判断是否有明确 3D 收益，再查官方文档与示例获取场景、材质、粒子和控制器；适合数字展厅、3D 产品和关系空间。",
        "tags": ["设计资源", "前端", "3D", "WebGL", "Three.js", "资源Skill"],
        "content": """# Three.js 3D 核心库

description: JavaScript 3D/WebGL 核心库，提供场景、相机、灯光、材质、粒子、模型和交互控制；当 3D 能明显改善理解、探索或展示时使用。

## When to use

- 数字展厅需要空间氛围、粒子、光柱或抽象装置；
- 重点产品/展品需要旋转、缩放或材质观察；
- 关系空间、知识星图或数据空间需要 2D 之外的空间布局；
- 真实材质、灯光、阴影或镜头移动是产品体验的一部分。

不用于：普通卡片、正文、搜索结果、数据表格或 CSS 已能清楚表达的效果。

## Source / license

- 官网：https://threejs.org/
- GitHub：https://github.com/mrdoob/three.js
- 许可证：MIT
- React 配套：`@react-three/fiber`、`@react-three/drei`；模型、纹理和 HDR 资产需单独确认许可。

## Agent reading workflow

1. 先读 description 和 `When to use`，判断是否真的需要 3D；
2. 再到 threejs.org/docs 或 examples 查官方类、示例和版本；
3. React 项目再查 R3F/Drei 文档，不直接复制原生 DOM 示例；
4. 记录几何体、材质、纹理、模型、GPU、内存和降级策略；
5. 实现懒加载、静态/CSS 降级、键盘入口和 reduced-motion；
6. 在桌面和真实移动设备上验证。

## Performance rules

- 粒子使用 `Points + BufferGeometry`；大量相同对象使用 `InstancedMesh`；
- 控制粒子数量、DPR、材质和 draw calls；
- `AdaptiveDpr` 只能调整分辨率，不能替代场景简化；
- 3D 初始化不能阻塞搜索、筛选、详情、表单和备份；
- 无 WebGL 时保留静态 Hero、CSS 光效和核心业务。

## Accessibility rules

- Canvas 添加 `role="img"` 和有意义的 `aria-label`；
- 纯装饰层可设置 `aria-hidden`；
- `prefers-reduced-motion` 要实时监听，停止自动旋转、粒子和镜头；
- 不能把 3D 作为唯一信息入口，必须有列表/文本替代。

## Prompt templates

- “从 Three.js/R3F/Drei 官方文档查一个低成本展厅方案，要求 Points、性能降级、无 WebGL 静态替代和 React 兼容性说明。”
- “评估关系空间使用 SVG、Canvas 还是 Three.js；先按节点数量、键盘、移动端和性能给结论，不要直接写代码。”

## Do not

不把个人信息或私密正文放入 3D 场景；不为炫酷引入未经评估的模型、后处理、物理引擎或视频序列；不让 WebGL 替代搜索、筛选和详情。
""",
    },
    {
        "title": "设计资源：GSAP 动画引擎",
        "old_id": "card-1787636319",
        "source": "https://gsap.com/",
        "summary": "通用时间轴与滚动动画引擎：先读 description 判断是否需要复杂编排，再到 GSAP 官方文档查 Timeline、ScrollTrigger、Flip 或 SplitText；简单交互优先 CSS/原生 API。",
        "tags": ["设计资源", "前端", "动效", "GSAP", "ScrollTrigger", "资源Skill"],
        "content": """# GSAP 动画引擎

description: 高性能 JavaScript 动画平台，提供 Timeline、ScrollTrigger、Flip、SplitText 和响应式媒体查询；适合复杂叙事与多段时间轴，不是所有 UI 都需要安装。

## When to use

- 多段动画需要精确时间轴；
- 滚动章节、视差、scrub 或 pin 形成内容叙事；
- 共享元素或多页面转场需要复杂编排；
- CSS、Web Animations API 或 View Transitions 无法稳定表达。

不用于：简单 hover、按钮反馈、普通列表淡入、弹窗和短状态提示。

## Source / license

- 官网：https://gsap.com/
- GitHub：https://github.com/greensock/GSAP
- 基础 GSAP 和插件的许可、商业授权边界以当前官方条款为准；SplitText 等插件需单独核对。
- 安装前记录版本、插件、包体、许可证和回滚方案。

## Component catalog

- Timeline：多段 Hero/章节/镜头同步；
- ScrollTrigger：scrub、reveal、parallax、pin；
- Flip：共享元素转场；
- SplitText：短标题分字，使用后 cleanup/revert；
- `gsap.matchMedia()`：响应式和 reduced-motion 分支。

## Agent reading workflow

1. 先读 description，确认原生 CSS/API 是否不足；
2. 到 gsap.com/docs 查具体插件和版本；
3. 只对短标题、装饰层和短时转场使用；
4. 不劫持主工作台原生滚动，不给正文做 parallax；
5. 使用 matchMedia 跳过 reduced-motion 非必要动画；
6. React 中使用明确 cleanup，避免重渲染泄漏。

## Prompt templates

- “从 GSAP 官方文档评估 ScrollTrigger 是否适合这个独立展厅实验，要求原生滚动、移动端简化和 reduced-motion 最终态。”
- “比较 CSS View Transitions、FLIP 和 GSAP Flip 的转场成本，只给出推荐和风险，不安装依赖。”

## Selection rules

- 复杂时间轴有明确收益后再引入；
- 视差只作用于装饰层，偏移保持小幅；
- 长列表每项延迟控制在 20–40ms；
- 不让动画延迟用户操作或掩盖内容层级。

## Verification

记录官方文档、插件许可、构建包体、移动设备性能、键盘、触屏、reduced-motion 和 cleanup 结果。
""",
    },
    {
        "title": "设计资源：Uiverse UI 社区",
        "old_id": "replace",
        "source": "https://uiverse.io/",
        "summary": "通用开源 UI 元素社区：先读 description 判断需要基础控件还是动效灵感，再按按钮、卡片、输入、加载器、开关等分类回到元素源页；MIT，可复制 HTML/CSS/Tailwind/React/Figma。",
        "tags": ["设计资源", "前端", "UI", "社区", "开源", "资源Skill"],
        "content": """# Uiverse UI 社区

description: 社区驱动的开源 UI 元素库，覆盖按钮、卡片、表单、输入、通知、加载器、开关、图案和工具提示；适合基础控件灵感和快速原型，不是完整应用设计系统。

## When to use

- 需要按钮、开关、输入框、加载器、Tooltip 或通知；
- 为空状态、导入状态或设置页补充短时反馈；
- 需要比较不同 CSS 交互，再按项目 Token 重写。

不用于：直接复制整页、覆盖项目设计系统、使用只有 hover 可见或只依赖颜色的状态。

## Source / license

- 官网：https://uiverse.io/（用户原地址 `uiuverse.com` 有误）
- GitHub：https://github.com/uiverse-io/galaxy
- 官方页面标注内容 MIT，支持个人/商业使用；具体元素取用时建议保留作者和源页信息。
- 输出格式：HTML/CSS、Tailwind、React、Figma。

## Component catalog

- Buttons：CTA、提交、刷新、导入、主题切换；
- Cards：空状态、辅助信息、装饰；
- Forms/Inputs：表单、筛选和设置；
- Notifications/Tooltips：成功、错误和状态解释；
- Loaders/Patterns：加载和背景降级；
- Toggles/Checkboxes：主题、减少动效和筛选设置。

## Agent reading workflow

1. 先读 description，确定基础控件还是装饰图案；
2. 回到具体元素源页查看 HTML/CSS、作者、许可和依赖；
3. 只复制必要结构和 CSS，改写为项目 Token；
4. 保留语义 HTML、键盘、focus、禁用、错误和触屏状态；
5. 通过主题、窄屏、reduced-motion 和真实操作验证。

## Prompt templates

- “在 Uiverse 官方源页检索适合这个主题的按钮/加载器，排除过度霓虹和无 focus 状态，输出源页、许可与改造建议。”
- “选择一个 Uiverse 减少动效开关，重写为语义 checkbox，支持键盘、三套主题和本地持久化。”

## Verification

记录具体元素 URL、作者/许可证、复制范围；必须通过键盘、窄屏、reduced-motion、主题切换和构建验证。
""",
    },
    {
        "title": "设计资源：Aura UI 复制粘贴组件库",
        "old_id": "card-1787636320",
        "source": "https://auraui.com/",
        "summary": "通用 React/Next.js 复制粘贴式动画组件和页面区块：先读 description 判断需要 Hero、导航、CTA 还是基础组件，再到官网源页读取具体实现；MIT，按需复制。",
        "tags": ["设计资源", "前端", "React", "Next.js", "组件库", "动效", "资源Skill"],
        "content": """# Aura UI 复制粘贴组件库

description: 面向 React/Next.js 的复制粘贴式 UI 资源，提供动画组件、导航、页脚、Header、Hero 和页面区块；适合快速原型、页面构图和高视觉区块研究。

## When to use

- 需要 Hero、导航、页脚、CTA 或页面区块原型；
- 需要比较 React/Next.js 动画组件的布局方式；
- 需要快速把视觉方向转换为可修改代码。

不用于：在 Vite/Tauri 项目中直接假设 Next.js API 可用；整套替换现有设计系统；未经适配直接复制 Tailwind 配置。

## Source / license

- 官网：https://auraui.com/
- GitHub：https://github.com/Shubham0850/auraui
- GitHub 项目公开标注 MIT；具体组件仍需回到官网源页确认依赖和更新时间。
- 注意同名项目：`aura-ui.com` 是 Laravel/Livewire PHP 组件库，不是本卡目标。

## Component catalog

- Hero / Header：首屏视觉入口、标题、CTA；
- Navbar / Footer：页面结构参考；
- Animated UI：按钮、卡片、背景和区块动效；
- Layout sections：专题页和实验页的构图参考。

## Agent reading workflow

1. 先读 description，确定是页面区块还是原子组件；
2. 到 auraui.com 的具体源页读取 React/Next.js 代码和 CSS/Tailwind 要求；
3. 当前项目优先抽取结构和动画逻辑，改写为原生 CSS/React；
4. 兼容性确认后再考虑依赖安装；
5. 验证键盘、focus、reduced-motion、窄屏和构建。

## Prompt templates

- “从 Aura UI 官方源页检索一个适合这个页面的 React Hero，只提取布局和动效，改写为现有技术栈，不安装整套库。”
- “比较 Aura UI 与 React Bits 的 CTA 组件，选择更适合效率型应用、不打断主要操作的方案。”

## Selection rules

- 页面区块先做独立草图，再进入正式页面；
- 复制组件必须改用项目 Token；
- 不把营销 Hero 结构套到搜索、管理、阅读页面；
- 页面动效必须有静态和 reduced-motion 状态。

## Verification

确认官网源页、GitHub 许可、React/Vite 兼容性、CSS/Tailwind 依赖、构建、键盘、窄屏和 reduced-motion。
""",
    },
]


def request(method, url, payload):
    req = urllib.request.Request(url, data=json.dumps(payload, ensure_ascii=False).encode(), headers={"Content-Type": "application/json"}, method=method)
    with urllib.request.urlopen(req, timeout=25) as resp:
        return json.loads(resp.read().decode())


def archive(cid):
    request("PATCH", f"{API}/{cid}", {"status": "已归档", "actor": "desktop-user"})
    print("archived", cid)
    time.sleep(1.2)


def update(resource):
    request("PATCH", f"{API}/{resource['old_id']}", {"summary": resource["summary"], "tags": resource["tags"], "content": resource["content"], "actor": "desktop-user"})
    print("updated", resource["title"])
    time.sleep(1.2)


def create(resource):
    body = {"title": resource["title"], "summary": resource["summary"], "type": "知识", "tags": resource["tags"], "source": resource["source"], "visibility": "私密", "status": "已验证", "content": resource["content"], "actor": "desktop-user"}
    result = request("POST", API, body)
    print("created", resource["title"], result.get("id"))
    time.sleep(1.3)


if __name__ == "__main__":
    # React Bits/Uiverse 旧卡片的 source 曾被占位地址污染，归档后按正确源地址重建。
    archive("card-1787636181")
    archive("card-1787636182")
    for resource in RESOURCES:
        if resource["old_id"] != "replace":
            update(resource)
    create(RESOURCES[5])
    create(RESOURCES[1])
