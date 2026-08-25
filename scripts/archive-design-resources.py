#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""批量归档前端设计资源到工作台「知识」栏目（POST /api/cards）"""
import json
import time
import urllib.request

API = "http://127.0.0.1:37821/api/cards"

RESOURCES = [
    {
        "title": "设计资源：Originkit 动画组件库",
        "type": "知识",
        "status": "已验证",
        "tags": ["设计资源", "前端", "动效", "动画组件", "粒子"],
        "source": "https://www.originkit.dev/",
        "summary": "免费动画组件库（Beta），约 50 个动画组件、6 大分类，含粒子特效、动态背景、文字动画、图片画廊与按钮交互；支持 React/Next.js/Vite/Framer，提供 CLI 与 MCP 注册表。",
        "content": """# Originkit — 免费动画组件库

- 官网：https://www.originkit.dev/
- 性质：免费动画组件库（Beta），MIT
- 规模：约 50 个动画组件，6 大分类

## 分类与代表组件
- Interactive Elements：Black Hole、Fluid Trail、SVG Particles、Kinetic Grid
- Image Gallery：Spiral Images、Coverflow Carousel、Swipe Stack、Infinity Canvas
- Text：Smoky Text、Text Morph、Scramble Text、Weight Hover
- Animation：Pixel Reveal、Star Burst、Particle Tunnel、Glitter Wrap
- Background Animation：Snow Fall、Blinking Squares、Character Waves
- Button：Emoji Burst、Link Preview

## 接入方式
- 网页端浏览复制
- CLI：`npx @originkit/cli@latest add <name>`（类 shadcn，需要登录/OAuth）
- MCP 注册表：`https://mcp.originkit.dev/v1`（list_components / get_component / search / fetch）
- 每日免费获取组件有限额

## 对本工作台的价值
- 粒子特效、动态背景、文字动效可用于 Hero 展厅与展品封面
- SVG Particles / Particle Tunnel 可作为 Three.js 氛围的 CSS/Canvas 降级方案
- 采用按需复制/改造，不整套安装；使用前核对许可证与依赖
""",
    },
    {
        "title": "设计资源：React Bits 动画组件库",
        "type": "知识",
        "status": "已验证",
        "tags": ["设计资源", "前端", "动效", "React", "交互组件"],
        "source": "https://www.reactbits.dev/",
        "summary": "开源动画与交互组件库（165+ 组件），含水波纹、动态照片墙、Spotlight Card、Tilted Card、Particles、Text Animations 等；支持复制源码按需改造。",
        "content": """# React Bits — 开源动画组件库

- 官网：https://www.reactbits.dev/
- 性质：开源，复制源码按需使用
- 规模：165+ 动画和交互组件

## 本工作台已借鉴/接入
- Spotlight Card：展品卡片鼠标聚光（已实现）
- Particles / Light Pillar：Hero 展厅氛围（已用 R3F 实现粒子层）
- Border Beam / Electric Border：状态边框光（已验证/收藏卡片已实现）
- Animated Content / CSS stagger：命令面板、版本、审计列表（已实现）
- Tilted Card / Glare Hover：重点展品（收藏卡片已实现）
- Scroll Expand：详情页展品转场（已用 View Transitions 实现）

## 常用组件方向
- Text Animations：SplitText、Gradual Spacing、Blur In
- Backgrounds：Aurora、Particles、Gradient
- Cards：Spotlight、Tilted、Glare
- Components：Animated List、Magnetic Button、Tabs

## 对本工作台的价值
- 组件可复制到 `src/components/` 按需改造
- 注意 React 19 兼容性与许可证（MIT + Commons Clause）
- 不整套安装，只复制具体组件
""",
    },
    {
        "title": "设计资源：Aceternity UI",
        "type": "知识",
        "status": "已验证",
        "tags": ["设计资源", "前端", "React", "组件库", "动效"],
        "source": "https://ui.aceternity.com/",
        "summary": "高质量 React 组件库：基础组件免费，完整模板需 Pro；含 Card Spotlight、Spotlight、3D Card、Moving Border、Animated Grid 等，适合数字展厅和高端视觉。",
        "content": """# Aceternity UI — 高质量 React 组件库

- 官网：https://ui.aceternity.com/
- 性质：基础组件免费，完整模板/Pro 需付费

## 本工作台关注组件
- Card Spotlight / Spotlight：卡片聚光（与 React Bits 思路一致，已用 Spotlight 实现）
- 3D Card / Tilt：展品 3D 倾斜（已用 CSS transform 实现）
- Moving Border / Border Beam：状态边框光（已验证/收藏卡片已实现）
- Animated Grid / Light Rays：背景与光效
- Sticky Scroll Reveal：滚动叙事

## 使用策略
- 只提炼交互效果和组件结构，先核对许可证与依赖
- 不引入完整库；按需复制/改造单个组件
- Pro 模板不采购，只参考公开免费部分的设计思路
""",
    },
    {
        "title": "设计资源：Three.js 3D 核心库",
        "type": "知识",
        "status": "已验证",
        "tags": ["设计资源", "前端", "3D", "WebGL", "Three.js"],
        "source": "https://threejs.org/",
        "summary": "3D 网页开发核心库，常用来做数字展厅、3D 产品展示、粒子与数据可视化；本工作台已安装 three + @react-three/fiber + @react-three/drei 并懒加载。",
        "content": """# Three.js — 3D 网页开发核心库

- 官网：https://threejs.org/
- 性质：开源（MIT）
- 配套：@react-three/fiber（React 渲染器）、@react-three/drei（辅助组件）

## 本工作台使用
- 已安装：`three@0.185`、`@react-three/fiber@9.7`、`@react-three/drei@10.7`
- 已实现：MuseumAtmosphere 懒加载粒子展厅（Points + BufferGeometry，约 90 点）
- Drei 已接入：Float、Sparkles、AdaptiveDpr
- 主包不直接引入 Three，独立 chunk 懒加载

## 性能门禁
- 粒子用 Points + BufferGeometry，不用大量独立 Mesh
- 粒子数量从 1000–3000 起步，实测移动端再上调
- 减少动效时粒子静止、Sparkles 为 0
- 无 WebGL 时保留静态 Hero、CSS 光效和核心业务
- Canvas 需语义说明（role=img + aria-label）

## 应用场景
- 数字展厅、3D 产品展示、知识星图、关系空间（草图见 sketches/relation-space/）
""",
    },
    {
        "title": "设计资源：GSAP 动画引擎",
        "type": "知识",
        "status": "已验证",
        "tags": ["设计资源", "前端", "动效", "GSAP", "ScrollTrigger"],
        "source": "https://gsap.com/",
        "summary": "行业级网页动画工具，大量主流网站使用；支持时间轴、ScrollTrigger 滚动叙事、SplitText 文字动画、Flip 转场；当前未安装，按需评估后使用。",
        "content": """# GSAP — 行业级动画引擎

- 官网：https://gsap.com/
- 性质：免费使用（保留许可说明），GreenSock 提供商业授权选项

## 主要能力
- Timeline：多段动画编排
- ScrollTrigger：滚动驱动动画、视差、钉住章节（Fin/MotionSites 风格）
- SplitText：标题分字动画（需注册，仅限短标题）
- Flip：共享元素转场（已用原生 View Transitions 替代基础需求）
- matchMedia：响应式 + prefers-reduced-motion 降级

## 与当前工作台的关系
- 当前状态：未安装（PRD 门禁：仅在滚动叙事或复杂时间轴确认收益后安装）
- 实验页 `sketches/scroll-story/` 使用原生 IntersectionObserver + rAF 实现，无 GSAP
- 若未来做复杂滚动叙事/镜头时间轴，再评估 `gsap` + `@gsap/react`

## 使用规范（来自 UI UX Pro Max 检索）
- 视差只作用于装饰层，不动正文
- 滚动 reveal 偏移保持 8–16px
- 长列表 stagger 每项 0.02–0.04s
- 减少动效下渲染最终状态
""",
    },
    {
        "title": "设计资源：Uiverse UI 社区",
        "type": "知识",
        "status": "已验证",
        "tags": ["设计资源", "前端", "UI", "社区", "开源"],
        "source": "https://uiverse.io/",
        "summary": "开源 UI 社区（注意正确域名是 uiverse.io），约 7400 款基础 UI 元素（按钮、卡片、表单、加载器、开关等），100% 个人/商业免费（MIT），可复制 HTML/CSS/Tailwind/React/Figma。",
        "content": """# Uiverse — 开源 UI 社区

- 官网：https://uiverse.io/（注意：不是 uiuverse.com）
- 性质：社区驱动开源 UI 元素库，MIT，100% 免费个人/商业使用
- 规模：约 7400 款基础 UI 元素，36 万+ 贡献者
- 输出格式：HTML/CSS、Tailwind、React、Figma

## 分类
- Buttons、Cards、Checkboxes、Forms、Inputs、Notifications、Patterns、Radio-buttons、Toggle-switches、Tooltips、Loaders

## 对本工作台的价值
- 基础 UI 元素灵感与快速原型（按钮、开关、加载器、卡片装饰）
- 适合补齐空状态、加载动画、设置页控件
- 复制后需按东方数字博物馆 Token 调整配色
- 官方镜像：github.com/uiverse-io/galaxy
""",
    },
    {
        "title": "设计资源：Aura UI 复制粘贴组件库",
        "type": "知识",
        "status": "已验证",
        "tags": ["设计资源", "前端", "React", "Next.js", "组件库", "动效"],
        "source": "https://auraui.com/",
        "summary": "React/Next.js 复制粘贴组件库（GitHub: Shubham0850/auraui，MIT），受 Magic UI / Aceternity / shadcn 启发，提供动画组件与导航、页脚、Hero 等页面区块，个人商业项目均可免费使用。",
        "content": """# Aura UI — React/Next.js 复制粘贴组件库

- 官网：https://auraui.com/
- GitHub：https://github.com/Shubham0850/auraui
- 性质：开源（MIT），复制粘贴使用，个人/商业免费
- 定位：受 Magic UI、Aceternity UI、ShadCN UI 启发的动画组件 + 页面区块

## 内容
- 动画组件：复制粘贴即可用
- 页面区块：导航栏、页脚、页眉、Hero 区块等
- 适合快速把设计稿转成代码

## 与本工作台的关系
- 可作为 Hero、导航、CTA、卡片区块的参考来源
- 与 React Bits / Aceternity 同类，按需复制/改造，不整套安装
- 注意与现有 React 19 + Tailwind 工程兼容性

## 注意：同名项目区分
- `aura-ui.com`：Laravel & Livewire 组件库（PHP 技术栈，不适用于本工作台）
- `auraui.com`（本卡片）：React/Next.js 组件库
- Product Hunt 另有 Figma + React 的 AI 产品组件库（不同项目）
""",
    },
]


def post_card(resource):
    body = {
        "title": resource["title"],
        "summary": resource["summary"],
        "type": resource["type"],
        "tags": resource["tags"],
        "source": resource["source"],
        "visibility": "私密",
        "status": resource["status"],
        "content": resource["content"],
        "actor": "desktop-user",
    }
    req = urllib.request.Request(
        API,
        data=json.dumps(body, ensure_ascii=False).encode("utf-8"),
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(req, timeout=20) as resp:
            data = json.loads(resp.read().decode("utf-8"))
            print("OK ", resource["title"], "->", data.get("id"))
            return True
    except Exception as exc:
        print("FAIL", resource["title"], exc)
        return False


if __name__ == "__main__":
    ok = 0
    for res in RESOURCES:
        if post_card(res):
            ok += 1
        time.sleep(0.3)
    print(f"created {ok}/{len(RESOURCES)}")
