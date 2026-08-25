export type CardType = '技能' | '知识' | '用户画像' | '偏好' | 'AI 产出' | '项目资料'
export type CardStatus = '已验证' | '待验证' | '草稿' | '已归档'

export interface Card {
  id: string
  title: string
  summary: string
  type: CardType
  tags: string[]
  source: string
  sourcePath?: string
  contentPath?: string
  visibility: '本机助手可读' | '仅工作台'
  status: CardStatus
  favorite: boolean
  accent: string
  content: string
  createdAt: string
  updatedAt: string
  deletedAt?: string | null
}

export const cardTypes: CardType[] = ['技能', '知识', '用户画像', '偏好', 'AI 产出', '项目资料']

export const typeMeta: Record<CardType, { icon: string; color: string }> = {
  技能: { icon: '✦', color: '#b69cff' },
  知识: { icon: '◈', color: '#66d9ef' },
  用户画像: { icon: '♧', color: '#f7a8d8' },
  偏好: { icon: '♡', color: '#ffb86b' },
  'AI 产出': { icon: '✧', color: '#91e6a1' },
  项目资料: { icon: '▣', color: '#ffd479' },
}

export const seedCards: Card[] = [
  {
    id: 'card-hermes-profile', title: 'Hermes 用户画像',
    summary: '关于我的稳定背景、求职方向与长期协作偏好，供本机 AI 助手参考。',
    type: '用户画像', tags: ['用户画像', '长期记忆', '求职'], source: 'Hermes 个人资料',
    visibility: '本机助手可读', status: '已验证', favorite: true, accent: '#f7a8d8',
    content: '# Hermes 用户画像\n\n- 目标方向：后端 + AI 应用 / Agent\n- 偏好城市：深圳、杭州、南昌\n- 学习方式：先独立思考，再使用 AI 辅助编程\n- 质量要求：代码必须能解释、运行、修改和测试\n',
    createdAt: '2026-08-24T09:00:00.000Z', updatedAt: '2026-08-24T09:00:00.000Z',
  },
  {
    id: 'card-local-first', title: '本地优先工作台设计原则',
    summary: '工作台保存技能、知识和 AI 产出，默认离线可用，助手只新增不覆盖。',
    type: '知识', tags: ['架构', '本地优先', '安全'], source: '与 AI 讨论产出',
    visibility: '本机助手可读', status: '已验证', favorite: true, accent: '#66d9ef',
    content: '# 本地优先工作台设计原则\n\n1. Markdown 保存正文，SQLite 保存索引。\n2. 原始文件和工作台副本同时保留。\n3. 助手默认可读，可新增，不可修改或删除已有卡片。\n4. 个人资料与密钥分离，密钥永不进入工作台。\n',
    createdAt: '2026-08-24T09:05:00.000Z', updatedAt: '2026-08-24T09:05:00.000Z',
  },
  {
    id: 'card-grilling', title: 'Grilling 设计访谈技能',
    summary: '通过设计树和分轮问题，把模糊想法逐步收敛为共享理解。',
    type: '技能', tags: ['Hermes', '设计', '访谈'], source: 'Hermes Skills Hub',
    visibility: '本机助手可读', status: '已验证', favorite: false, accent: '#b69cff',
    content: '# Grilling\n\n用于复杂产品设计与决策。每一轮询问当前 frontier，等待用户回答后再展开下一层。\n',
    createdAt: '2026-08-24T09:10:00.000Z', updatedAt: '2026-08-24T09:10:00.000Z',
  },
  {
    id: 'card-workbench-vision', title: '个人 AI 产出与知识资产工作台',
    summary: '一个不运行内容、主要负责存储、检索、阅读和向助手提供上下文的独立桌面应用。',
    type: 'AI 产出', tags: ['产品构想', '桌面应用', '知识库'], source: '本次设计讨论',
    visibility: '本机助手可读', status: '草稿', favorite: false, accent: '#91e6a1',
    content: '# 工作台愿景\n\n它是我的 AI 资产柜：技能、知识卡片、用户画像、偏好、AI 产出和项目资料都可以沉淀在这里。\n',
    createdAt: '2026-08-24T09:15:00.000Z', updatedAt: '2026-08-24T09:15:00.000Z',
  },
]
