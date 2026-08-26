import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { Search, Plus, LayoutDashboard, Library, Sparkles, BookOpen, UserRound, Heart, Clock3, Trash2, RotateCcw, Archive, Settings2, SlidersHorizontal, Tag, FileText, ExternalLink, Star, X, ArrowUpRight, ShieldCheck, Database, Upload, Menu, ChevronRight, RefreshCw, FolderOpen, Palette, Command } from 'lucide-react'
import { Card, CardType, cardTypes, typeMeta } from './types'
import museumHero from './assets/museum/eastern-digital-library-hero.webp'
import { lazy, Suspense } from 'react'
const MuseumAtmosphere = lazy(() => import('./components/MuseumAtmosphere'))
const ExhibitPreview = lazy(() => import('./components/ExhibitPreview'))
const RelationSpace = lazy(() => import('./components/RelationSpace'))
import BrandLogo from './components/BrandLogo'

function loadAtmosphereWhenIdle() { return new Promise<void>(resolve => { const idle = (window as Window & { requestIdleCallback?: (callback: () => void, options?: { timeout: number }) => number }).requestIdleCallback; if (idle) idle(() => resolve(), { timeout: 1800 }); else window.setTimeout(resolve, 700) }) }

const nav = [
  { id: '总览', icon: LayoutDashboard }, { id: '全部内容', icon: Library },
  { id: '技能', icon: Sparkles }, { id: '知识', icon: BookOpen },
  { id: '项目资料', icon: FolderOpen }, { id: 'AI 产出', icon: FileText },
  { id: '收藏', icon: Heart }, { id: '最近更新', icon: Clock3 }, { id: '回收站', icon: Trash2 },
]

function formatLocalDate(value: string | number) { const raw = String(value); const numeric = /^\d+$/.test(raw) ? Number(raw) : NaN; const date = Number.isFinite(numeric) ? new Date(numeric < 10_000_000_000 ? numeric * 1000 : numeric) : new Date(raw); if (Number.isNaN(date.getTime())) return '时间未知'; return new Intl.DateTimeFormat(undefined, { year:'numeric', month:'2-digit', day:'2-digit', hour:'2-digit', minute:'2-digit' }).format(date) }
function getVersionChange(version: { summary: string; status: string }, current: Card) { if (version.summary === current.summary && version.status === current.status) return '与当前版本相比无摘要字段变化'; if (version.status !== current.status) return `状态：${current.status} → ${version.status}`; return version.summary ? `摘要：${version.summary}` : '正文或标签发生变化'; }

const API = 'http://127.0.0.1:37821/api'
type ThemePreset = 'night' | 'paper' | 'amber'
const themeAccent: Record<ThemePreset, string> = { night: '#8e7cff', paper: '#3b9e91', amber: '#e0ad68' }

function App() {
  const [cards, setCards] = useState<Card[]>([])
  const [active, setActive] = useState('总览')
  const [query, setQuery] = useState('')
  const [searchDraft, setSearchDraft] = useState('')
  const [selected, setSelected] = useState<Card | null>(null)
  const [tagFilter, setTagFilter] = useState('全部标签')
  const [statusFilter, setStatusFilter] = useState('全部状态')
  const [showComposer, setShowComposer] = useState(false)
  const [mobileNav, setMobileNav] = useState(false)
  const [notice, setNotice] = useState('')
  const [noticeTone, setNoticeTone] = useState<'success' | 'error' | 'info'>('success')
  const [loading, setLoading] = useState(true)
  const [loadError, setLoadError] = useState('')
  const [theme, setTheme] = useState<ThemePreset>(() => (localStorage.getItem('workbench-theme') as ThemePreset) || 'night')
  const [appName, setAppName] = useState(() => { const saved = localStorage.getItem('workbench-app-name'); return !saved || saved === '我的工作台' ? '工作台' : saved })
  const [accent, setAccent] = useState(() => localStorage.getItem('workbench-accent') || '#b69cff')
  const [reducedMotion, setReducedMotion] = useState(() => localStorage.getItem('workbench-reduced-motion') === 'true')
  const [systemReducedMotion, setSystemReducedMotion] = useState(() => window.matchMedia('(prefers-reduced-motion: reduce)').matches)
  const [showCommandPalette, setShowCommandPalette] = useState(false)
  const [atmosphereReady, setAtmosphereReady] = useState(false)
  const themeDetailsRef = useRef<HTMLDetailsElement>(null)

  useEffect(() => {
    document.documentElement.dataset.theme = theme
    document.documentElement.style.setProperty('--user-accent', accent)
    document.documentElement.style.setProperty('--theme-accent', accent)
    document.documentElement.classList.toggle('reduce-motion', reducedMotion || systemReducedMotion)
    localStorage.setItem('workbench-theme', theme)
    localStorage.setItem('workbench-app-name', appName)
    localStorage.setItem('workbench-accent', accent)
    localStorage.setItem('workbench-reduced-motion', String(reducedMotion))
  }, [theme, appName, accent, reducedMotion, systemReducedMotion])
  useEffect(() => { const media = window.matchMedia('(prefers-reduced-motion: reduce)'); const onChange = () => setSystemReducedMotion(media.matches); media.addEventListener('change', onChange); return () => media.removeEventListener('change', onChange) }, [])
  useEffect(() => { const onKeyDown = (event: KeyboardEvent) => { if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === 'k') { event.preventDefault(); setShowCommandPalette(true) } if (event.key === 'Escape') { setShowCommandPalette(false); if (themeDetailsRef.current) themeDetailsRef.current.open = false } }; window.addEventListener('keydown', onKeyDown); return () => window.removeEventListener('keydown', onKeyDown) }, [])
  useEffect(() => { const onPointerDown = (event: PointerEvent) => { if (themeDetailsRef.current?.open && !themeDetailsRef.current.contains(event.target as Node)) themeDetailsRef.current.open = false }; document.addEventListener('pointerdown', onPointerDown); return () => document.removeEventListener('pointerdown', onPointerDown) }, [])


  const showNotice = (message: string, tone: 'success' | 'error' | 'info' = 'success') => { setNoticeTone(tone); setNotice(message); window.setTimeout(() => setNotice(''), 2800) }
  const loadCards = useCallback(async () => {
    setLoading(true)
    setLoadError('')
    try {
      const response = await fetch(`${API}/cards?include_deleted=1`)
      if (!response.ok) throw new Error('API unavailable')
      const data = await response.json() as unknown
      if (!Array.isArray(data)) throw new Error('本地数据格式错误')
      setCards(data as Card[])
    } catch {
      try {
        await invoke('seed_workspace')
        const response = await fetch(`${API}/cards?include_deleted=1`)
        const fallback = await response.json() as unknown
        if (!Array.isArray(fallback)) throw new Error('本地数据格式错误')
        setCards(fallback as Card[])
      } catch { setLoadError('暂时无法读取本地数据，请确认桌面后端已启动'); showNotice('本地数据暂时不可用') }
    } finally { setLoading(false) }
  }, [])
  useEffect(() => { void loadCards() }, [loadCards])
  useEffect(() => { if (active !== '总览' || atmosphereReady) return; let cancelled = false; void loadAtmosphereWhenIdle().then(() => { if (!cancelled) setAtmosphereReady(true) }); return () => { cancelled = true } }, [active, atmosphereReady])

  const allTags = useMemo(() => ['全部标签', ...Array.from(new Set(cards.flatMap((card) => card.tags)))], [cards])
  const allStatuses = ['全部状态', '草稿', '待验证', '已验证', '已归档']
  const filtered = useMemo(() => cards.filter((card) => {
    const byNav = active === '总览' || active === '全部内容' || active === '设置' || (active === '回收站' ? Boolean(card.deletedAt) : !card.deletedAt && (active === '收藏' ? card.favorite : active === '最近更新' ? true : card.type === active))
    const haystack = `${card.title} ${card.summary} ${card.content} ${card.tags.join(' ')} ${card.source}`.toLowerCase()
    return byNav && (!query.trim() || haystack.includes(query.trim().toLowerCase())) && (tagFilter === '全部标签' || card.tags.includes(tagFilter)) && (statusFilter === '全部状态' || card.status === statusFilter)
  }).sort((a, b) => b.updatedAt.localeCompare(a.updatedAt)), [cards, active, query, tagFilter, statusFilter])
  const stats = useMemo(() => ({ total: cards.filter(c => !c.deletedAt).length, skills: cards.filter(c => !c.deletedAt && c.type === '技能').length, notes: cards.filter(c => !c.deletedAt && c.type === '知识').length, favorites: cards.filter(c => !c.deletedAt && c.favorite).length, recycle: cards.filter(c => Boolean(c.deletedAt)).length }), [cards])

  const createCard = async (card: Card) => {
    try {
      const response = await fetch(`${API}/cards`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ title: card.title, summary: card.summary, type: card.type, tags: card.tags, source: card.source, visibility: card.visibility, status: card.status, content: card.content, actor: 'desktop-user' }) })
      if (!response.ok) throw new Error(await response.text())
      await loadCards(); setShowComposer(false); showNotice('已新增卡片并保存到 MySQL')
    } catch { showNotice('新增失败：请确认本地数据服务可用') }
  }
  const toggleFavorite = async (card: Card) => {
    setCards(prev => prev.map(item => item.id === card.id ? { ...item, favorite: !card.favorite } : item))
    try {
      const response = await fetch(`${API}/cards/${encodeURIComponent(card.id)}`, { method: 'PATCH', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ favorite: !card.favorite, actor: 'desktop-user' }) })
      if (!response.ok) throw new Error()
    } catch { setCards(prev => prev.map(item => item.id === card.id ? { ...item, favorite: card.favorite } : item)); showNotice('收藏状态保存失败') }
  }
  const changeDeletion = async (card: Card) => {
    try {
      await invoke(card.deletedAt ? 'restore_card' : 'soft_delete_card', { id: card.id })
      await loadCards()
      setSelected(null)
      showNotice(card.deletedAt ? '卡片已恢复' : '卡片已移入回收站')
    } catch { showNotice('操作失败：卡片状态未改变') }
  }
  const archiveCard = async (card: Card) => {
    try {
      const response = await fetch(`${API}/cards/${encodeURIComponent(card.id)}`, { method: 'PATCH', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ status: card.status === '已归档' ? '草稿' : '已归档', actor: 'desktop-user' }) })
      if (!response.ok) throw new Error()
      await loadCards(); setSelected(null); showNotice(card.status === '已归档' ? '卡片已取消归档' : '卡片已归档')
    } catch { showNotice('归档操作失败') }
  }
  const importFiles = async () => {
    try {
      const selected = await open({ multiple: true, filters: [{ name: '文档', extensions: ['md', 'txt', 'pdf', 'docx', 'xlsx'] }] })
      if (!selected) return
      const paths = Array.isArray(selected) ? selected : [selected]
      let imported = 0
      const failures: string[] = []
      for (const path of paths) {
        try {
          const response = await fetch(`${API}/import`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ path, actor: 'desktop-user' }) })
          if (response.ok) imported += 1
          else failures.push(`${path.split(/[\\/]/).pop()}: ${(await response.text()).slice(0, 120)}`)
        } catch { failures.push(`${path.split(/[\\/]/).pop()}: 无法连接本地导入服务`) }
      }
      await loadCards()
      if (imported > 0 && failures.length === 0) showNotice(`已导入 ${imported} 个文件，并记录原始路径`)
      else if (imported > 0) showNotice(`已导入 ${imported} 个文件；失败 ${failures.length} 个：${failures[0]}`, 'info')
      else showNotice(failures[0] || '没有可导入的文件', 'error')
    } catch { showNotice('导入失败：无法访问所选文件') }
  }
  const resetFilters = () => { setQuery(''); setSearchDraft(''); setTagFilter('全部标签'); setStatusFilter('全部状态') }
  const executeSearch = () => setQuery(searchDraft.trim())

  return <div className="app-shell">
    <aside className={`sidebar ${mobileNav ? 'mobile-open' : ''}`}>
      <div className="brand"><div className="brand-mark"><BrandLogo size={30} /></div><div><strong>{appName}</strong><span>PERSONAL AI DESK</span></div><button className="mobile-close" onClick={() => setMobileNav(false)}><X size={18} /></button></div>
      <div className="workspace-pill"><div className="avatar">H</div><div><b>{appName}</b><small>本地优先 · 私人</small></div><ChevronRight size={15} /></div>
      <nav className="nav-list">{nav.map(({ id, icon: Icon }) => <button key={id} title={id === '回收站' ? '查看软删除卡片，可恢复或永久删除' : undefined} className={`nav-item ${active === id ? 'active' : ''}`} onClick={() => { setActive(id); setMobileNav(false) }}><Icon size={17} /><span>{id}</span>{id === '全部内容' && <em>{stats.total}</em>}{id === '回收站' && <em>{stats.recycle}</em>}</button>)}</nav>
      <div className="sidebar-section"><span className="section-label">快捷入口</span><button className="nav-item" onClick={() => resetFilters()}><Tag size={17} /><span>标签浏览</span></button><button className="nav-item" onClick={() => setActive('设置')}><Settings2 size={17} /><span>工作台设置</span></button></div>
      <div className="sidebar-bottom"><div className="storage-status"><span className="status-dot" /> <span>本地存储正常</span><small>MySQL · Windows 凭据</small></div><div className="profile-row"><div className="profile-avatar">H</div><div><b>我的个人空间</b><small>仅本机可见</small></div><SlidersHorizontal size={16} /></div></div>
    </aside>
    {mobileNav && <div className="mobile-scrim" onClick={() => setMobileNav(false)} />}
    <main className="main-content">
      <header className="topbar"><button className="mobile-menu" onClick={() => setMobileNav(true)}><Menu size={21} /></button><div className="breadcrumb"><span>{active === '总览' ? 'Workspace' : 'Library'}</span><ChevronRight size={14} /><b>{active}</b></div><div className="top-actions"><button className="icon-button"><ShieldCheck size={17} /><span>本机助手可读</span></button><details ref={themeDetailsRef} className="theme-details"><summary className="quiet-button"><Palette size={16} /> 主题</summary><ThemeMenu theme={theme} accent={accent} reducedMotion={reducedMotion} onTheme={setTheme} onAccent={setAccent} onReducedMotion={setReducedMotion} /></details><button className="quiet-button command-trigger" onClick={() => setShowCommandPalette(true)}><Command size={16} /> 命令 <kbd>⌘ K</kbd></button><button className="quiet-button" onClick={() => void importFiles()}><FolderOpen size={16} /> 导入</button><button className="new-button" onClick={() => setShowComposer(true)}><Plus size={17} /> 新建卡片</button></div></header>
      <section className="hero museum-hero" onMouseMove={event => { if (reducedMotion || systemReducedMotion) return; const rect = event.currentTarget.getBoundingClientRect(); event.currentTarget.style.setProperty('--hero-shift-x', `${((event.clientX - rect.left) / rect.width - .5) * 10}px`); event.currentTarget.style.setProperty('--hero-shift-y', `${((event.clientY - rect.top) / rect.height - .5) * 6}px`) }} style={active === '总览' ? { '--museum-hero': `url(${museumHero})` } as React.CSSProperties : undefined}>{active === '总览' && atmosphereReady && <Suspense fallback={<div className="museum-atmosphere-fallback" aria-hidden="true" />}><MuseumAtmosphere reducedMotion={reducedMotion || systemReducedMotion} theme={theme} /></Suspense>}<div className="museum-hero-content"><div className="eyebrow"><span className="eyebrow-dot" /> EASTERN DIGITAL LIBRARY</div><h1>{active === '总览' ? <>把重要的东西，<i>放在一起。</i></> : active}</h1><p>{active === '总览' ? '在本机统一管理、检索、版本化和恢复你的 AI 知识资产。' : `在 ${active} 中找到你需要的内容。`}</p></div><div className="hero-space-grid" aria-hidden="true" /><div className="hero-orbit"><div className="orbit-ring ring-one" /><div className="orbit-ring ring-two" /><div className="orbit-ring ring-three" /><div className="orbit-core">✦</div></div></section>
      {active === '设置' ? <SettingsView appName={appName} onAppName={value => setAppName(value.trim() || '我的工作台')} onNotice={showNotice} /> : <><section className="toolbar"><div className="search-box"><Search size={18} /><input value={searchDraft} onChange={e => setSearchDraft(e.target.value)} onKeyDown={e => { if (e.key === 'Enter') executeSearch() }} placeholder="搜索工作台中的内容..." />{searchDraft && <button aria-label="清空搜索" onClick={() => { setSearchDraft(''); setQuery('') }}><X size={15} /></button>}<button className="search-submit" aria-label="查询内容" onClick={executeSearch}>查询</button><kbd>⌘ K</kbd></div><div className="filter-row"><div className="select-wrap"><Tag size={15} /><select value={tagFilter} onChange={e => setTagFilter(e.target.value)}>{allTags.map(tag => <option key={tag}>{tag}</option>)}</select></div><div className="select-wrap"><select value={statusFilter} onChange={e => setStatusFilter(e.target.value)}>{allStatuses.map(status => <option key={status}>{status}</option>)}</select></div><span className="result-count">找到 {filtered.length} 张卡片</span></div></section>
      {active === '总览' && !query && <section className="stats-row"><Stat label="全部资产" value={stats.total} icon={<Database size={16} />} tone="purple" /><Stat label="技能卡片" value={stats.skills} icon={<Sparkles size={16} />} tone="pink" /><Stat label="知识沉淀" value={stats.notes} icon={<BookOpen size={16} />} tone="blue" /><Stat label="我的收藏" value={stats.favorites} icon={<Heart size={16} />} tone="orange" /></section>}
      <section className="section-heading"><div><span className="eyebrow">{active === '总览' ? 'RECENTLY IN YOUR DESK' : 'YOUR COLLECTION'}</span><h2>{query ? `搜索 “${query}”` : active === '总览' ? '最近沉淀' : active}</h2></div><div className="heading-actions"><button className="quiet-button" onClick={resetFilters}><SlidersHorizontal size={15} /> 清除筛选</button><button className="quiet-button" onClick={() => void loadCards()}><RefreshCw size={15} /> 刷新</button></div></section>
      <section className="card-grid">{loading ? <StateCard kind="loading" onAction={() => void loadCards()} /> : loadError ? <StateCard kind="error" onAction={() => void loadCards()} /> : filtered.map((card, index) => <CardItem key={card.id} card={card} index={index} featured={card.favorite} reducedMotion={reducedMotion || systemReducedMotion} onOpen={() => { const doc = document as Document & { startViewTransition?: (callback: () => void) => void }; if (typeof doc.startViewTransition === 'function') { doc.startViewTransition(() => setSelected(card)) } else { setSelected(card) } }} onFavorite={() => void toggleFavorite(card)} />)}{!loading && !loadError && filtered.length === 0 && <StateCard kind={query || tagFilter !== '全部标签' || statusFilter !== '全部状态' ? 'empty' : 'no-content'} onAction={resetFilters} />}</section></>}
    </main>
    {selected && <DetailPanel card={selected} availableCards={cards} reducedMotion={reducedMotion || systemReducedMotion} onClose={() => setSelected(null)} onDelete={() => void changeDeletion(selected)} onArchive={() => void archiveCard(selected)} />}
    {showCommandPalette && <CommandPalette query={query} onQuery={setQuery} onClose={() => setShowCommandPalette(false)} onNavigate={setActive} onTheme={() => { setTheme(theme === 'night' ? 'paper' : theme === 'paper' ? 'amber' : 'night'); setShowCommandPalette(false) }} onNew={() => { setShowComposer(true); setShowCommandPalette(false) }} onRefresh={() => { void loadCards(); setShowCommandPalette(false) }} onFilter={(filter) => { if (filter.active) setActive(filter.active); if (filter.status) setStatusFilter(filter.status); if (filter.tag) setTagFilter(filter.tag); setShowCommandPalette(false) }} />}
    {showComposer && <Composer onClose={() => setShowComposer(false)} onCreate={createCard} />}
    {notice && <div className={`toast toast-${noticeTone}`}><span className="toast-orbit" aria-hidden="true" /><div><b>{noticeTone === 'success' ? '已完成' : noticeTone === 'error' ? '需要注意' : '正在处理'}</b><span>{notice}</span></div><i aria-hidden="true" /></div>}
  </div>
}

function Stat({ label, value, icon, tone }: { label: string; value: number; icon: React.ReactNode; tone: string }) { return <div className={`stat-card ${tone}`}><div className="stat-icon">{icon}</div><div><span>{label}</span><strong>{value}</strong></div><ArrowUpRight size={15} /></div> }
function StateCard({ kind, onAction }: { kind: 'loading' | 'error' | 'empty' | 'no-content'; onAction: () => void }) { const states = { loading: { icon: <RefreshCw className="spin" size={28} />, title: '正在打开东方数字图书馆', text: '正在从本机 MySQL 读取展品。', action: '重新读取' }, error: { icon: <ShieldCheck size={28} />, title: '展厅暂时无法打开', text: '本地数据服务没有返回有效内容。数据没有被修改。', action: '重试' }, empty: { icon: <Search size={28} />, title: '没有找到对应展品', text: '可以调整关键词、标签或状态筛选。', action: '清除筛选' }, 'no-content': { icon: <Sparkles size={28} />, title: '展厅还没有展品', text: '可以新建一张卡片，或导入本地文档开始收藏。', action: '新建/导入' } }[kind]; return <div className={`empty-state state-${kind}`}><div className="empty-icon">{states.icon}</div><h3>{states.title}</h3><p>{states.text}</p><button className="quiet-button" onClick={onAction}>{states.action}</button></div> }
function hslToHex(h: number, s = 78, l = 68) { const c = (1 - Math.abs(2 * l / 100 - 1)) * s / 100; const x = c * (1 - Math.abs((h / 60) % 2 - 1)); const m = l / 100 - c / 2; const rgb = h < 60 ? [c, x, 0] : h < 120 ? [x, c, 0] : h < 180 ? [0, c, x] : h < 240 ? [0, x, c] : h < 300 ? [x, 0, c] : [c, 0, x]; return `#${rgb.map(value => Math.round((value + m) * 255).toString(16).padStart(2, '0')).join('')}` }
function hueFromHex(hex: string) { const value = hex.replace('#', ''); if (value.length !== 6) return 260; const [r, g, b] = [0, 2, 4].map(index => parseInt(value.slice(index, index + 2), 16) / 255); const max = Math.max(r, g, b); const min = Math.min(r, g, b); const d = max - min; if (!d) return 0; const hue = max === r ? ((g - b) / d) % 6 : max === g ? (b - r) / d + 2 : (r - g) / d + 4; return Math.round((hue * 60 + 360) % 360) }
function AccentPicker({ accent, onAccent }: { accent: string; onAccent: (value: string) => void }) { const [hue, setHue] = useState(() => hueFromHex(accent)); const presets = [['#8e7cff', '星紫'], ['#29d8ff', '电青'], ['#ff5aa9', '霓粉'], ['#e8bd70', '琥珀'], ['#48d6a0', '青玉']]; const changeHue = (value: number) => { setHue(value); onAccent(hslToHex(value)) }; return <div className="accent-picker"><div className="accent-picker-head"><span>主色调</span><span className="accent-value"><i style={{ background: accent }} />{accent.toUpperCase()}</span></div><div className="accent-presets">{presets.map(([value, label]) => <button key={value} className={accent.toLowerCase() === value ? 'active' : ''} aria-label={`使用${label}色`} title={label} style={{ '--swatch': value } as React.CSSProperties} onClick={() => { setHue(hueFromHex(value)); onAccent(value) }} />)}</div><input className="hue-slider" aria-label="调整主色调" type="range" min="0" max="359" value={hue} onChange={event => changeHue(Number(event.target.value))} /></div> }
function ThemeMenu({ theme, accent, reducedMotion, onTheme, onAccent, onReducedMotion }: { theme: ThemePreset; accent: string; reducedMotion: boolean; onTheme: (value: ThemePreset) => void; onAccent: (value: string) => void; onReducedMotion: (value: boolean) => void }) { return <div className="theme-menu" onClick={event => event.stopPropagation()}><div className="theme-menu-title"><span>视觉设置</span></div><div className="theme-presets">{([['night', '暗夜墨蓝'], ['paper', '暖白纸张'], ['amber', '琥珀暖色']] as [ThemePreset, string][]).map(([value, label]) => <button key={value} className={theme === value ? 'selected' : ''} onClick={() => { onTheme(value); onAccent(themeAccent[value]) }}><span className={`theme-swatch ${value}`} />{label}</button>)}</div><AccentPicker accent={accent} onAccent={onAccent} /><label className="motion-control"><input type="checkbox" checked={reducedMotion} onChange={event => onReducedMotion(event.target.checked)} />减少动效</label><small>设置仅保存在当前本机浏览器。</small></div> }
function CommandPalette({ query, onQuery, onClose, onNavigate, onTheme, onNew, onRefresh, onFilter }: { query: string; onQuery: (value: string) => void; onClose: () => void; onNavigate: (value: string) => void; onTheme: () => void; onNew: () => void; onRefresh: () => void; onFilter: (filter: { active?: string; status?: string; tag?: string }) => void }) { const actions = [{ label: '浏览全部内容', hint: '导航', run: () => onNavigate('全部内容') }, { label: '打开技能', hint: '导航', run: () => onNavigate('技能') }, { label: '打开知识', hint: '导航', run: () => onNavigate('知识') }, { label: '只看收藏', hint: '筛选', run: () => onFilter({ active: '收藏' }) }, { label: '只看已验证', hint: '筛选', run: () => onFilter({ status: '已验证' }) }, { label: '查看回收站', hint: '筛选', run: () => onFilter({ active: '回收站' }) }, { label: '切换主题', hint: '视觉', run: onTheme }, { label: '新建卡片', hint: '操作', run: onNew }, { label: '刷新本地数据', hint: '数据', run: onRefresh }]; const [term, setTerm] = useState(query); const visible = actions.filter(action => !term || `${action.label} ${action.hint}`.toLowerCase().includes(term.toLowerCase())); return <div className="command-scrim" onClick={onClose}><section className="command-palette" onClick={event => event.stopPropagation()}><div className="command-input"><Command size={17} /><span className="command-orbit" aria-hidden="true" /><input autoFocus value={term} onChange={event => { setTerm(event.target.value); onQuery(event.target.value) }} placeholder="输入命令或搜索卡片..." /><kbd>ESC</kbd></div><div className="command-list">{visible.map((action, index) => <button key={action.label} style={{ '--stagger': `${Math.min(index, 10) * 35}ms` } as React.CSSProperties} className="stagger-item" onClick={action.run}><span>{action.label}</span><small>{action.hint}</small></button>)}{visible.length === 0 && <p>没有匹配的命令，按 Esc 关闭。</p>}</div></section></div> }
function getCardMeta(card: Card) { return typeMeta[card.type] ?? typeMeta['项目资料'] }
function exhibitImage(type: CardType) { return undefined }
function CardItem({ card, index, onOpen, onFavorite, featured, reducedMotion }: { card: Card; index: number; onOpen: () => void; onFavorite: () => void; featured: boolean; reducedMotion: boolean }) { const meta = getCardMeta(card); const beam = card.favorite ? 'beam-gold' : card.status === '已验证' ? 'beam-jade' : ''; return <SpotlightCard className={`content-card atlas-card atlas-${card.type} ${beam} ${featured ? 'featured-tilt' : ''}`} style={{ '--stagger': `${Math.min(index, 8) * 70}ms` } as React.CSSProperties} spotlightColor={`${card.accent || meta.color}55`} featured={featured} reducedMotion={reducedMotion} onClick={onOpen}><span className="card-flow" aria-hidden="true" /><div className="card-top"><div className="type-badge"><span style={{ color: meta.color }}>{meta.icon}</span>{card.type || '未分类'}</div><button aria-label={card.favorite ? '取消收藏' : '收藏'} className={`star-button ${card.favorite ? 'is-favorite' : ''}`} onClick={e => { e.stopPropagation(); onFavorite() }}><Star size={16} fill={card.favorite ? 'currentColor' : 'none'} /></button></div><h3>{card.title}</h3><p>{card.summary}</p><div className="tag-list">{card.tags.slice(0, 3).map(tag => <span key={tag}>#{tag}</span>)}</div><div className="card-bottom"><span className="source"><span className="source-dot" />{card.source}</span><span>{formatLocalDate(card.updatedAt)}</span></div></SpotlightCard> }
function SpotlightCard({ children, className, spotlightColor, onClick, style, featured, reducedMotion }: { children: React.ReactNode; className?: string; spotlightColor: string; onClick?: () => void; style?: React.CSSProperties; featured?: boolean; reducedMotion?: boolean }) { const ref = useRef<HTMLElement>(null); const handleMove = (event: React.MouseEvent<HTMLElement>) => { if (!ref.current) return; const rect = ref.current.getBoundingClientRect(); const x = event.clientX - rect.left; const y = event.clientY - rect.top; ref.current.style.setProperty('--spot-x', `${x}px`); ref.current.style.setProperty('--spot-y', `${y}px`); ref.current.style.setProperty('--color-x', `${(x / rect.width) * 100}%`); ref.current.style.setProperty('--color-y', `${(y / rect.height) * 100}%`); if (featured && !reducedMotion) { const dx = x / rect.width - 0.5; const dy = y / rect.height - 0.5; ref.current.style.setProperty('--tilt-x', `${dy * -6}deg`); ref.current.style.setProperty('--tilt-y', `${dx * 6}deg`) } }; return <article ref={ref} className={`spotlight-card ${className || ''}`} style={{ '--spotlight-color': spotlightColor, ...style } as React.CSSProperties} onMouseMove={handleMove} onClick={onClick}>{children}</article> }

function DetailPanel({ card, availableCards, reducedMotion, onClose, onDelete, onArchive }: { card: Card; availableCards: Card[]; reducedMotion: boolean; onClose: () => void; onDelete: () => void; onArchive: () => void }) { const meta = getCardMeta(card); const [versions, setVersions] = useState<Array<{ id: number; summary: string; status: string; createdAt: string }>>([]); useEffect(() => { fetch(`${API}/cards/${encodeURIComponent(card.id)}/versions`).then(r => r.ok ? r.json() : []).then(setVersions).catch(() => setVersions([])) }, [card.id]); const restoreVersion = async (versionId: number) => { if (!window.confirm('确定恢复这个版本吗？恢复前会自动保存当前版本。')) return; try { const response = await fetch(`${API}/cards/${encodeURIComponent(card.id)}/versions/${versionId}/restore`, { method: 'POST' }); if (!response.ok) throw new Error(); window.location.reload() } catch { window.alert('版本恢复失败，请稍后重试。') } }; const permanentlyDelete = async () => { if (!card.deletedAt || !window.confirm('永久删除后无法恢复，确定继续吗？')) return; try { await invoke('permanently_delete_card', { id: card.id }); onClose(); window.location.reload() } catch { window.alert('永久删除失败，卡片未被删除。') } }; return <><div className="panel-scrim" onClick={onClose} /><aside className="detail-panel detail-page"><div className="panel-header"><button className="quiet-button" onClick={onClose}>← 返回资产图鉴</button><span className="status-label"><span className="status-dot" />{card.deletedAt ? '回收站' : card.status}</span></div><div className="detail-heading"><Suspense fallback={<div className="exhibit-preview exhibit-preview-loading" aria-hidden="true" /> }><ExhibitPreview color={card.favorite ? '#e8bd70' : meta.color} reducedMotion={reducedMotion} /></Suspense><div className="type-badge"><span style={{ color: meta.color }}>{meta.icon}</span>{card.type || '未分类'}</div><h2>{card.title}</h2><p>{card.summary}</p><div className="detail-badges"><span>{card.favorite ? '★ 已收藏' : '☆ 未收藏'}</span><span>更新时间 {formatLocalDate(card.updatedAt)}</span></div></div><div className="detail-meta"><div><span>来源</span><b>{card.source}</b></div><div><span>可见范围</span><b>{card.visibility}</b></div>{card.contentPath && <div><span>本地正文文件</span><b className="path-value">{card.contentPath}</b></div>}{card.sourcePath && <div><span>原始文件绝对路径</span><b className="path-value">{card.sourcePath}</b></div>}</div><section className="detail-section"><span className="eyebrow">CONTENT</span><div className="markdown-preview"><Markdown text={card.content} /></div><div className="detail-tags">{card.tags.map(tag => <span key={tag}>#{tag}</span>)}</div></section><section className="detail-section"><details className="version-details"><summary><span className="eyebrow">VERSION HISTORY</span><b>{versions.length} 个版本</b></summary>{versions.length === 0 ? <small className="muted-copy">暂无历史版本</small> : <div className="version-list">{versions.map((version, index) => <div className="version-row stagger-item" style={{ '--stagger': `${Math.min(index, 10) * 40}ms` } as React.CSSProperties} key={version.id}><div><b>版本 {version.id}</b><small>{formatLocalDate(version.createdAt)} · {version.status}</small><p>{getVersionChange(version, card)}</p></div><button className="quiet-button" onClick={() => void restoreVersion(version.id)}>恢复</button></div>)}</div>}</details></section><RelatedSection cardId={card.id} availableCards={availableCards} /><div className="panel-footer"><button className="quiet-button" title="打开这张卡片的原始来源"><ExternalLink size={15} /> 查看原始来源</button><button className="quiet-button" title={card.deletedAt ? '恢复这张软删除卡片' : '移入回收站，可在回收站中恢复'} onClick={onDelete}>{card.deletedAt ? <RotateCcw size={15} /> : <Trash2 size={15} />}{card.deletedAt ? '恢复卡片' : '移入回收站'}</button>{!card.deletedAt && <button className="quiet-button" title="保留数据，但从日常列表隐藏；可随时取消归档" onClick={() => void onArchive()}>{card.status === '已归档' ? <RotateCcw size={15} /> : <Archive size={15} />}{card.status === '已归档' ? '取消归档' : '归档'}</button>}{card.deletedAt && <button className="quiet-button danger-button" title="永久删除后无法恢复" onClick={() => void permanentlyDelete()}><Trash2 size={15} /> 永久删除</button>}</div></aside></> }
function RelatedSection({ cardId, availableCards }: { cardId: string; availableCards: Card[] }) {
  const [relations, setRelations] = useState<Array<{ cardId: string; relationType: string; title?: string; type?: string }>>([])
  const [targetId, setTargetId] = useState('')
  const [relationType, setRelationType] = useState('相关知识')
  const [message, setMessage] = useState('')
  const [showSpace, setShowSpace] = useState(false)
  const loadRelations = () => { fetch(`${API}/cards/${encodeURIComponent(cardId)}/relations`).then(r => r.ok ? r.json() : []).then(setRelations).catch(() => setRelations([])) }
  useEffect(() => { loadRelations() }, [cardId])
  const addRelation = async () => { if (!targetId) return; const response = await fetch(`${API}/cards/${encodeURIComponent(cardId)}/relations`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ to_card_id: targetId, relation_type: relationType, actor: 'desktop-user' }) }); if (response.ok) { setMessage('关联已添加'); setTargetId(''); loadRelations() } else { const text = await response.text(); setMessage(text.includes('关系已存在') ? '关系已存在' : '关联添加失败') } }
  return <section className="related-section"><div className="related-heading"><span className="eyebrow">RELATED CARDS</span><button className="quiet-button space-button" onClick={() => setShowSpace(value => !value)}>{showSpace ? '关闭关系空间' : '打开关系空间'}</button></div>{showSpace && <Suspense fallback={<div className="relation-space-loading">正在加载关系空间…</div>}><RelationSpace relations={relations} onSelect={relation => setMessage(`已聚焦：${relation.title || relation.cardId}`)} /></Suspense>}{relations.length === 0 ? <small>暂无关联卡片</small> : relations.map(relation => <div className="related-card" key={`${relation.cardId}-${relation.relationType}`}><span>{relation.relationType}</span><b>{relation.title || relation.cardId}</b><small>{relation.type || '卡片'}</small></div>)}{availableCards.length > 0 && <div className="relation-form"><select value={targetId} onChange={event => setTargetId(event.target.value)}><option value="">选择目标卡片</option>{availableCards.filter(item => item.id !== cardId && !item.deletedAt).map(item => <option key={item.id} value={item.id}>{item.title}</option>)}</select><select value={relationType} onChange={event => setRelationType(event.target.value)}>{['关联项目', '相关知识', '来源于', '依赖'].map(type => <option key={type}>{type}</option>)}</select><button className="quiet-button" onClick={() => void addRelation()}>添加关联</button>{message && <small>{message}</small>}</div>}</section>
}

function Markdown({ text }: { text: string }) { return <div>{text.split('\n').map((line, i) => line.startsWith('# ') ? <h3 key={i}>{line.slice(2)}</h3> : line.startsWith('- ') ? <div className="md-list" key={i}>• {line.slice(2)}</div> : <p key={i}>{line || '\u00a0'}</p>)}</div> }

function SettingsView({ onNotice, appName, onAppName }: { onNotice: (message: string) => void; appName: string; onAppName: (value: string) => void }) {
  const [nameDraft, setNameDraft] = useState(appName)
  const [audit, setAudit] = useState<Array<{ id: number; actor: string; action: string; targetId: string; detail: string; createdAt: string }>>([])
  const [auditLoading, setAuditLoading] = useState(true)
  const [backupState, setBackupState] = useState<'idle' | 'running' | 'done' | 'error'>('idle')
  const loadAudit = () => { setAuditLoading(true); fetch(`${API}/audit`).then(r => r.ok ? r.json() : []).then(data => { if (Array.isArray(data)) setAudit(data) }).catch(() => setAudit([])).finally(() => setAuditLoading(false)) }
  useEffect(() => { loadAudit() }, [])
  const runBackup = async () => { setBackupState('running'); try { const response = await fetch(`${API}/backup`, { method: 'POST' }); if (!response.ok) throw new Error(); setBackupState('done'); onNotice('备份已创建到本地 backups 目录') } catch { setBackupState('error'); onNotice('备份失败，请稍后重试') } }
  return <section className="settings-view">
    <section className="settings-card"><div className="settings-card-head"><span className="eyebrow">WORKSPACE IDENTITY</span><h2>应用名称</h2><p className="muted-copy">自定义桌面工作台名称，保存后会在侧边栏和下次启动时保留。</p></div><div className="name-setting-row"><input value={nameDraft} maxLength={40} onChange={event => setNameDraft(event.target.value)} placeholder="例如：小明的工作台" /><button className="quiet-button" onClick={() => { onAppName(nameDraft); onNotice('应用名称已保存') }}>保存名称</button></div></section>
    <section className="settings-card"><div className="settings-card-head"><span className="eyebrow">DATA SAFETY</span><h2>本地数据安全</h2></div><p className="muted-copy">MySQL 保存卡片、正文、版本、关系、审计和文件对象；本地仍保留迁移回滚副本与备份。</p><div className="settings-actions"><button className="quiet-button" onClick={() => void runBackup()} disabled={backupState === 'running'}>{backupState === 'running' ? '正在备份…' : '立即创建 ZIP 备份'}</button>{backupState === 'done' && <span className="settings-ok">备份完成</span>}{backupState === 'error' && <span className="settings-warn">备份失败</span>}</div></section>
    <section className="settings-card"><div className="settings-card-head"><span className="eyebrow">AUDIT LOG</span><h2>操作审计</h2><p className="muted-copy">记录每一次新增、修改、导入、备份和恢复操作。AI 助手永远不能删除卡片。</p></div>{auditLoading ? <div className="settings-loading">读取审计记录…</div> : audit.length === 0 ? <div className="settings-empty">暂无审计记录</div> : <div className="audit-list">{audit.slice(0, 12).map((entry, index) => <div className="audit-row stagger-item" style={{ '--stagger': `${Math.min(index, 10) * 30}ms` } as React.CSSProperties} key={entry.id}><span className={`audit-action audit-${entry.action}`}>{entry.action}</span><b>{entry.targetId}</b><small>{entry.actor} · {entry.createdAt}</small></div>)}</div>}<button className="quiet-button settings-refresh" onClick={() => void loadAudit()}><RefreshCw size={14} /> 刷新审计</button></section>
    <section className="settings-card"><div className="settings-card-head"><span className="eyebrow">IMPORT</span><h2>导入状态</h2></div><ul className="settings-list"><li>支持 Markdown、TXT、PDF、DOCX、XLSX 只读文本提取；</li><li>导入后生成“待验证”卡片，原始文件副本保存在本机 originals 目录；</li><li>原始文件不会被修改；真实个人数据不会上传到云端。</li></ul></section>
  </section>
}
function Composer({ onClose, onCreate }: { onClose: () => void; onCreate: (card: Card) => void }) { const [title, setTitle] = useState(''); const [type, setType] = useState<CardType>('知识'); const [tags, setTags] = useState(''); const [content, setContent] = useState(''); const submit = () => { if (!title.trim()) return; const now = new Date().toISOString(); onCreate({ id: `local-${Date.now()}`, title: title.trim(), summary: content.trim().split('\n')[0] || '由 AI 助手或我新增的工作台内容。', type, tags: tags.split(/[,，]/).map(t => t.trim()).filter(Boolean), source: '手动新增', visibility: '本机助手可读', status: '草稿', favorite: false, accent: typeMeta[type].color, content: content || `# ${title.trim()}\n\n`, createdAt: now, updatedAt: now }) }; return <><div className="panel-scrim" onClick={onClose} /><div className="composer"><div className="composer-head"><div><span className="eyebrow">NEW ASSET</span><h2>新建一张卡片</h2></div><button className="close-button" onClick={onClose}><X size={18} /></button></div><label>标题<input autoFocus value={title} onChange={e => setTitle(e.target.value)} placeholder="例如：FastAPI 错误处理清单" /></label><div className="form-grid"><label>类型<select value={type} onChange={e => setType(e.target.value as CardType)}>{cardTypes.map(t => <option key={t}>{t}</option>)}</select></label><label>标签<input value={tags} onChange={e => setTags(e.target.value)} placeholder="用逗号分隔" /></label></div><label>正文<textarea value={content} onChange={e => setContent(e.target.value)} placeholder="支持 Markdown，先写下最重要的内容..." /></label><div className="composer-note"><ShieldCheck size={15} /> 新增内容会保存为草稿；助手只能新增或按权限修改，不能删除已有卡片。</div><div className="composer-actions"><button className="quiet-button" onClick={onClose}>取消</button><button className="new-button" disabled={!title.trim()} onClick={submit}><Plus size={16} /> 保存卡片</button></div></div></> }

export default App
